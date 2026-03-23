// =============================================================================
// Medication Queries
// Ported from packages/db/src/services/medication.service.ts (424 lines)
// =============================================================================

use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::enums::{DayOfWeek, InstructionTiming, MedicationEventStatus, MedicationFrequency};
use bominal_types::models::{Medication, MedicationEvent, MedicationSchedule};
use bominal_types::state_machines::medication_event_machine;

// ---------------------------------------------------------------------------
// Input structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CreateMedicationData {
    pub person_id: Uuid,
    pub name: String,
    pub dosage: String,
    pub form: String,
    pub frequency: MedicationFrequency,
    pub prescribed_by: Option<String>,
    pub prescribed_at: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub side_effects: Option<String>,
    pub notes: Option<String>,
    pub instruction_timing: Option<InstructionTiming>,
    pub instruction_minutes: Option<i32>,
    pub instruction_text: Option<String>,
    pub total_quantity: Option<i32>,
    pub doses_per_intake: Option<i32>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateMedicationData {
    pub name: Option<String>,
    pub dosage: Option<String>,
    pub form: Option<String>,
    pub frequency: Option<MedicationFrequency>,
    pub prescribed_by: Option<String>,
    pub prescribed_at: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
    pub side_effects: Option<String>,
    pub notes: Option<String>,
    pub instruction_timing: Option<InstructionTiming>,
    pub instruction_minutes: Option<i32>,
    pub instruction_text: Option<String>,
    pub total_quantity: Option<i32>,
    pub doses_per_intake: Option<i32>,
    pub updated_by: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub struct AddScheduleData {
    pub time_of_day: String,
    pub day_of_week: Option<DayOfWeek>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct DateRange {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MedicationWithSchedules {
    pub medication: Medication,
    pub schedules: Vec<MedicationSchedule>,
}

/// Intermediate row for the schedule-join query in `generate_events`.
#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
struct ScheduleRow {
    pub sched_id: Uuid,
    pub time_of_day: String,
    pub day_of_week: Option<DayOfWeek>,
}

/// Intermediate row for the event + person_id join in `update_event_status`.
#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
struct EventWithPersonId {
    pub id: Uuid,
    pub medication_id: Uuid,
    pub scheduled_for: DateTime<Utc>,
    pub status: MedicationEventStatus,
    pub taken_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub recorded_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub person_id: Uuid,
}

// ---------------------------------------------------------------------------
// create_medication
// ---------------------------------------------------------------------------

pub async fn create_medication(
    pool: &PgPool,
    data: &CreateMedicationData,
) -> Result<Medication, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query_as::<_, Medication>(
        "INSERT INTO medications (
           id, person_id, name, dosage, form, frequency,
           prescribed_by, prescribed_at, start_date, end_date,
           is_active, side_effects, notes,
           instruction_timing, instruction_minutes, instruction_text,
           total_quantity, doses_per_intake,
           created_by, created_at, updated_at
         ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, true, $11, $12, $13, $14, $15, $16, $17, $18, $19, $19)
         RETURNING *",
    )
    .bind(id)
    .bind(data.person_id)
    .bind(&data.name)
    .bind(&data.dosage)
    .bind(&data.form)
    .bind(data.frequency)
    .bind(&data.prescribed_by)
    .bind(data.prescribed_at)
    .bind(data.start_date)
    .bind(data.end_date)
    .bind(&data.side_effects)
    .bind(&data.notes)
    .bind(data.instruction_timing)
    .bind(data.instruction_minutes)
    .bind(&data.instruction_text)
    .bind(data.total_quantity)
    .bind(data.doses_per_intake.unwrap_or(1))
    .bind(data.created_by)
    .bind(now)
    .fetch_one(pool)
    .await
}

// ---------------------------------------------------------------------------
// get_medication
// ---------------------------------------------------------------------------

pub async fn get_medication(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<MedicationWithSchedules>, sqlx::Error> {
    let medication = sqlx::query_as::<_, Medication>(
        "SELECT * FROM medications WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    let medication = match medication {
        Some(m) => m,
        None => return Ok(None),
    };

    let schedules = sqlx::query_as::<_, MedicationSchedule>(
        "SELECT * FROM medication_schedules WHERE medication_id = $1 AND is_active = true ORDER BY time_of_day ASC",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    Ok(Some(MedicationWithSchedules {
        medication,
        schedules,
    }))
}

// ---------------------------------------------------------------------------
// update_medication
// ---------------------------------------------------------------------------

pub async fn update_medication(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateMedicationData,
) -> Result<Medication, sqlx::Error> {
    // Verify existence
    sqlx::query_as::<_, Medication>(
        "SELECT * FROM medications WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let now = Utc::now();

    // Use COALESCE pattern: each optional field only overwrites when the bind
    // is non-NULL, otherwise the existing column value is preserved.
    sqlx::query_as::<_, Medication>(
        "UPDATE medications SET
           name               = COALESCE($1,  name),
           dosage             = COALESCE($2,  dosage),
           form               = COALESCE($3,  form),
           frequency          = COALESCE($4,  frequency),
           prescribed_by      = COALESCE($5,  prescribed_by),
           prescribed_at      = COALESCE($6,  prescribed_at),
           start_date         = COALESCE($7,  start_date),
           end_date           = COALESCE($8,  end_date),
           is_active          = COALESCE($9,  is_active),
           side_effects       = COALESCE($10, side_effects),
           notes              = COALESCE($11, notes),
           instruction_timing = COALESCE($12, instruction_timing),
           instruction_minutes = COALESCE($13, instruction_minutes),
           instruction_text   = COALESCE($14, instruction_text),
           total_quantity     = COALESCE($15, total_quantity),
           doses_per_intake   = COALESCE($16, doses_per_intake),
           updated_by         = COALESCE($17, updated_by),
           updated_at         = $18
         WHERE id = $19
         RETURNING *",
    )
    .bind(&data.name)
    .bind(&data.dosage)
    .bind(&data.form)
    .bind(data.frequency)
    .bind(&data.prescribed_by)
    .bind(data.prescribed_at)
    .bind(data.start_date)
    .bind(data.end_date)
    .bind(data.is_active)
    .bind(&data.side_effects)
    .bind(&data.notes)
    .bind(data.instruction_timing)
    .bind(data.instruction_minutes)
    .bind(&data.instruction_text)
    .bind(data.total_quantity)
    .bind(data.doses_per_intake)
    .bind(data.updated_by)
    .bind(now)
    .bind(id)
    .fetch_one(pool)
    .await
}

// ---------------------------------------------------------------------------
// add_schedule
// ---------------------------------------------------------------------------

pub async fn add_schedule(
    pool: &PgPool,
    medication_id: Uuid,
    data: &AddScheduleData,
) -> Result<MedicationSchedule, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let is_active = data.is_active.unwrap_or(true);

    sqlx::query_as::<_, MedicationSchedule>(
        "INSERT INTO medication_schedules (id, medication_id, time_of_day, day_of_week, is_active, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $6)
         RETURNING *",
    )
    .bind(id)
    .bind(medication_id)
    .bind(&data.time_of_day)
    .bind(data.day_of_week)
    .bind(is_active)
    .bind(now)
    .fetch_one(pool)
    .await
}

// ---------------------------------------------------------------------------
// generate_events
// ---------------------------------------------------------------------------

/// Generates `MedicationEvent` records for every active schedule within the
/// given date range, skipping duplicates via `ON CONFLICT DO NOTHING`.
pub async fn generate_events(
    pool: &PgPool,
    medication_id: Uuid,
    date_range: &DateRange,
) -> Result<Vec<MedicationEvent>, sqlx::Error> {
    // Fetch active schedules for this medication
    let schedules = sqlx::query_as::<_, ScheduleRow>(
        "SELECT ms.id AS sched_id, ms.time_of_day, ms.day_of_week
         FROM medications m
         JOIN medication_schedules ms ON ms.medication_id = m.id AND ms.is_active = true
         WHERE m.id = $1",
    )
    .bind(medication_id)
    .fetch_all(pool)
    .await?;

    if schedules.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    // Map chrono weekday index to our DayOfWeek enum
    let day_of_week_for = |date: &NaiveDate| -> DayOfWeek {
        match date.weekday() {
            chrono::Weekday::Mon => DayOfWeek::Monday,
            chrono::Weekday::Tue => DayOfWeek::Tuesday,
            chrono::Weekday::Wed => DayOfWeek::Wednesday,
            chrono::Weekday::Thu => DayOfWeek::Thursday,
            chrono::Weekday::Fri => DayOfWeek::Friday,
            chrono::Weekday::Sat => DayOfWeek::Saturday,
            chrono::Weekday::Sun => DayOfWeek::Sunday,
        }
    };

    // Collect (medication_id, scheduled_for) tuples to insert
    let mut events_to_insert: Vec<DateTime<Utc>> = Vec::new();

    let start_date = date_range.from.date_naive();
    let end_date = date_range.to.date_naive();
    let mut current = start_date;

    while current <= end_date {
        let day_name = day_of_week_for(&current);

        for schedule in &schedules {
            if let Some(required_day) = schedule.day_of_week
                && required_day != day_name {
                    continue;
                }

            // Parse "HH:MM" time_of_day
            let time = NaiveTime::parse_from_str(&schedule.time_of_day, "%H:%M")
                .unwrap_or_else(|_| NaiveTime::from_hms_opt(0, 0, 0).unwrap());

            let naive_dt = current.and_time(time);
            let scheduled_for = DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc);
            events_to_insert.push(scheduled_for);
        }

        current += chrono::Duration::days(1);
    }

    if events_to_insert.is_empty() {
        return Ok(Vec::new());
    }

    // Insert events in a transaction, skipping duplicates
    let mut tx = pool.begin().await?;
    let now = Utc::now();

    for scheduled_for in &events_to_insert {
        let evt_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO medication_events (id, medication_id, scheduled_for, status, created_at, updated_at)
             VALUES ($1, $2, $3, 'SCHEDULED', $4, $4)
             ON CONFLICT (medication_id, scheduled_for) DO NOTHING",
        )
        .bind(evt_id)
        .bind(medication_id)
        .bind(scheduled_for)
        .bind(now)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    // Return all events in range
    sqlx::query_as::<_, MedicationEvent>(
        "SELECT * FROM medication_events
         WHERE medication_id = $1
           AND scheduled_for >= $2
           AND scheduled_for <= $3
         ORDER BY scheduled_for ASC",
    )
    .bind(medication_id)
    .bind(date_range.from)
    .bind(date_range.to)
    .fetch_all(pool)
    .await
}

// ---------------------------------------------------------------------------
// update_event_status
// ---------------------------------------------------------------------------

pub async fn update_event_status(
    pool: &PgPool,
    event_id: Uuid,
    status: MedicationEventStatus,
    recorded_by: Option<Uuid>,
    notes: Option<&str>,
) -> Result<MedicationEvent, sqlx::Error> {
    // Fetch event with person_id for state-machine validation
    let existing = sqlx::query_as::<_, EventWithPersonId>(
        "SELECT me.*, m.person_id
         FROM medication_events me
         JOIN medications m ON m.id = me.medication_id
         WHERE me.id = $1",
    )
    .bind(event_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let machine = medication_event_machine();
    if !machine.can_transition(existing.status, status) {
        return Err(sqlx::Error::Protocol(format!(
            "Invalid medication event transition: {} -> {}",
            existing.status, status
        )));
    }

    let now = Utc::now();
    let taken_at: Option<DateTime<Utc>> = if status == MedicationEventStatus::Taken {
        Some(now)
    } else {
        None
    };

    sqlx::query_as::<_, MedicationEvent>(
        "UPDATE medication_events SET
           status      = $1,
           taken_at    = COALESCE($2, taken_at),
           recorded_by = COALESCE($3, recorded_by),
           notes       = COALESCE($4, notes),
           updated_at  = $5
         WHERE id = $6
         RETURNING *",
    )
    .bind(status)
    .bind(taken_at)
    .bind(recorded_by)
    .bind(notes)
    .bind(now)
    .bind(event_id)
    .fetch_one(pool)
    .await
}

// ---------------------------------------------------------------------------
// get_today_events
// ---------------------------------------------------------------------------

pub async fn get_today_events(
    pool: &PgPool,
    person_id: Uuid,
) -> Result<Vec<MedicationEvent>, sqlx::Error> {
    let today = Utc::now().date_naive();
    let today_start = today
        .and_hms_opt(0, 0, 0)
        .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
        .unwrap();
    let today_end = today
        .and_hms_milli_opt(23, 59, 59, 999)
        .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
        .unwrap();

    sqlx::query_as::<_, MedicationEvent>(
        "SELECT me.*
         FROM medication_events me
         JOIN medications m ON m.id = me.medication_id
         WHERE m.person_id = $1
           AND m.is_active = true
           AND me.scheduled_for >= $2
           AND me.scheduled_for <= $3
         ORDER BY me.scheduled_for ASC",
    )
    .bind(person_id)
    .bind(today_start)
    .bind(today_end)
    .fetch_all(pool)
    .await
}

// ---------------------------------------------------------------------------
// get_overdue_events
// ---------------------------------------------------------------------------

pub async fn get_overdue_events(
    pool: &PgPool,
    person_id: Uuid,
) -> Result<Vec<MedicationEvent>, sqlx::Error> {
    let now = Utc::now();

    sqlx::query_as::<_, MedicationEvent>(
        "SELECT me.*
         FROM medication_events me
         JOIN medications m ON m.id = me.medication_id
         WHERE m.person_id = $1
           AND m.is_active = true
           AND me.scheduled_for < $2
           AND me.status IN ('MISSED', 'SCHEDULED', 'REMINDER_SENT')
         ORDER BY me.scheduled_for ASC",
    )
    .bind(person_id)
    .bind(now)
    .fetch_all(pool)
    .await
}

// ---------------------------------------------------------------------------
// list_medications (with schedules)
// ---------------------------------------------------------------------------

pub async fn list_medications(
    pool: &PgPool,
    person_id: Uuid,
) -> Result<Vec<MedicationWithSchedules>, sqlx::Error> {
    let medications = sqlx::query_as::<_, Medication>(
        "SELECT * FROM medications WHERE person_id = $1 AND is_active = true ORDER BY name ASC",
    )
    .bind(person_id)
    .fetch_all(pool)
    .await?;

    let schedules = sqlx::query_as::<_, MedicationSchedule>(
        "SELECT ms.* FROM medication_schedules ms
         JOIN medications m ON m.id = ms.medication_id
         WHERE m.person_id = $1 AND m.is_active = true AND ms.is_active = true",
    )
    .bind(person_id)
    .fetch_all(pool)
    .await?;

    // Group schedules by medication_id
    let mut scheds_by_med: std::collections::HashMap<Uuid, Vec<MedicationSchedule>> =
        std::collections::HashMap::new();
    for sched in schedules {
        scheds_by_med
            .entry(sched.medication_id)
            .or_default()
            .push(sched);
    }

    let result = medications
        .into_iter()
        .map(|med| {
            let schedules = scheds_by_med.remove(&med.id).unwrap_or_default();
            MedicationWithSchedules {
                medication: med,
                schedules,
            }
        })
        .collect();

    Ok(result)
}

// ---------------------------------------------------------------------------
// update_schedule_reminder
// ---------------------------------------------------------------------------

pub async fn update_schedule_reminder(
    pool: &PgPool,
    schedule_id: Uuid,
    reminder_enabled: bool,
    reminder_minutes_before: i32,
) -> Result<MedicationSchedule, sqlx::Error> {
    sqlx::query_as::<_, MedicationSchedule>(
        "UPDATE medication_schedules SET reminder_enabled = $1, reminder_minutes_before = $2, updated_at = NOW() WHERE id = $3 RETURNING *"
    )
    .bind(reminder_enabled)
    .bind(reminder_minutes_before)
    .bind(schedule_id)
    .fetch_one(pool)
    .await
}
