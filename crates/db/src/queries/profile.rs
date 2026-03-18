// =============================================================================
// Profile queries — PersonProfile and SeniorProfile CRUD
// Ported from packages/db/src/services/profile.service.ts
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::{KycLevel, UserRole};

// -----------------------------------------------------------------------------
// Input types
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersonProfileData {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub national_id: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePersonProfileData {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub national_id: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSeniorProfileData {
    pub person_id: Uuid,
    pub mobility_level: Option<String>,
    pub cognitive_level_score: Option<i32>,
    pub preferred_language: Option<String>,
    pub dietary_restrictions: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSeniorProfileData {
    pub mobility_level: Option<String>,
    pub cognitive_level_score: Option<i32>,
    pub preferred_language: Option<String>,
    pub dietary_restrictions: Option<String>,
    pub notes: Option<String>,
}

// -----------------------------------------------------------------------------
// Row types for joined queries
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummary {
    pub id: Uuid,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub role: UserRole,
    pub kyc_level: KycLevel,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonProfileWithUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub national_id: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user: UserSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummaryShort {
    pub id: Uuid,
    pub email: Option<String>,
    pub name: Option<String>,
    pub role: UserRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonProfileSummary {
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub national_id: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user: UserSummaryShort,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeniorProfileWithPerson {
    pub id: Uuid,
    pub person_id: Uuid,
    pub mobility_level: Option<String>,
    pub cognitive_level_score: Option<i32>,
    pub preferred_language: Option<String>,
    pub dietary_restrictions: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub person_profile: PersonProfileSummary,
}

// -----------------------------------------------------------------------------
// Flat row types used by sqlx::query_as for joined queries
// -----------------------------------------------------------------------------

#[derive(Debug, sqlx::FromRow)]
struct PersonProfileUserRow {
    // person_profiles.*
    id: Uuid,
    user_id: Uuid,
    first_name: String,
    last_name: String,
    date_of_birth: Option<DateTime<Utc>>,
    gender: Option<String>,
    national_id: Option<String>,
    phone: Option<String>,
    address: Option<String>,
    city: Option<String>,
    district: Option<String>,
    emergency_contact_name: Option<String>,
    emergency_contact_phone: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    // users (aliased)
    u_id: Uuid,
    u_email: Option<String>,
    u_name: Option<String>,
    u_phone: Option<String>,
    u_role: UserRole,
    u_kyc_level: KycLevel,
    u_is_active: bool,
}

impl PersonProfileUserRow {
    fn into_profile_with_user(self) -> PersonProfileWithUser {
        PersonProfileWithUser {
            id: self.id,
            user_id: self.user_id,
            first_name: self.first_name,
            last_name: self.last_name,
            date_of_birth: self.date_of_birth,
            gender: self.gender,
            national_id: self.national_id,
            phone: self.phone,
            address: self.address,
            city: self.city,
            district: self.district,
            emergency_contact_name: self.emergency_contact_name,
            emergency_contact_phone: self.emergency_contact_phone,
            created_at: self.created_at,
            updated_at: self.updated_at,
            user: UserSummary {
                id: self.u_id,
                email: self.u_email,
                name: self.u_name,
                phone: self.u_phone,
                role: self.u_role,
                kyc_level: self.u_kyc_level,
                is_active: self.u_is_active,
            },
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct SeniorProfileJoinRow {
    // senior_profiles.*
    id: Uuid,
    person_id: Uuid,
    mobility_level: Option<String>,
    cognitive_level_score: Option<i32>,
    preferred_language: Option<String>,
    dietary_restrictions: Option<String>,
    notes: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    // person_profiles (aliased)
    pp_id: Uuid,
    pp_user_id: Uuid,
    pp_first_name: String,
    pp_last_name: String,
    pp_date_of_birth: Option<DateTime<Utc>>,
    pp_gender: Option<String>,
    pp_national_id: Option<String>,
    pp_phone: Option<String>,
    pp_address: Option<String>,
    pp_city: Option<String>,
    pp_district: Option<String>,
    pp_emergency_contact_name: Option<String>,
    pp_emergency_contact_phone: Option<String>,
    pp_created_at: DateTime<Utc>,
    pp_updated_at: DateTime<Utc>,
    // users (aliased)
    u_id: Uuid,
    u_email: Option<String>,
    u_name: Option<String>,
    u_role: UserRole,
}

impl SeniorProfileJoinRow {
    fn into_senior_with_person(self) -> SeniorProfileWithPerson {
        SeniorProfileWithPerson {
            id: self.id,
            person_id: self.person_id,
            mobility_level: self.mobility_level,
            cognitive_level_score: self.cognitive_level_score,
            preferred_language: self.preferred_language,
            dietary_restrictions: self.dietary_restrictions,
            notes: self.notes,
            created_at: self.created_at,
            updated_at: self.updated_at,
            person_profile: PersonProfileSummary {
                id: self.pp_id,
                user_id: self.pp_user_id,
                first_name: self.pp_first_name,
                last_name: self.pp_last_name,
                date_of_birth: self.pp_date_of_birth,
                gender: self.pp_gender,
                national_id: self.pp_national_id,
                phone: self.pp_phone,
                address: self.pp_address,
                city: self.pp_city,
                district: self.pp_district,
                emergency_contact_name: self.pp_emergency_contact_name,
                emergency_contact_phone: self.pp_emergency_contact_phone,
                created_at: self.pp_created_at,
                updated_at: self.pp_updated_at,
                user: UserSummaryShort {
                    id: self.u_id,
                    email: self.u_email,
                    name: self.u_name,
                    role: self.u_role,
                },
            },
        }
    }
}

// -----------------------------------------------------------------------------
// PersonProfile row (for INSERT / UPDATE RETURNING *)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PersonProfileRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub national_id: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// -----------------------------------------------------------------------------
// SeniorProfile row (for INSERT / UPDATE RETURNING *)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SeniorProfileRow {
    pub id: Uuid,
    pub person_id: Uuid,
    pub mobility_level: Option<String>,
    pub cognitive_level_score: Option<i32>,
    pub preferred_language: Option<String>,
    pub dietary_restrictions: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

pub async fn create_person_profile(
    pool: &PgPool,
    data: &CreatePersonProfileData,
) -> Result<PersonProfileRow, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query_as::<_, PersonProfileRow>(
        "INSERT INTO person_profiles (
           id, user_id, first_name, last_name, date_of_birth, gender, national_id,
           phone, address, city, district, emergency_contact_name, emergency_contact_phone,
           created_at, updated_at
         ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)
         RETURNING *",
    )
    .bind(id)
    .bind(data.user_id)
    .bind(&data.first_name)
    .bind(&data.last_name)
    .bind(data.date_of_birth)
    .bind(&data.gender)
    .bind(&data.national_id)
    .bind(&data.phone)
    .bind(&data.address)
    .bind(&data.city)
    .bind(&data.district)
    .bind(&data.emergency_contact_name)
    .bind(&data.emergency_contact_phone)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await
}

pub async fn update_person_profile(
    pool: &PgPool,
    id: Uuid,
    data: &UpdatePersonProfileData,
) -> Result<PersonProfileRow, sqlx::Error> {
    // Verify the profile exists
    let exists: Option<(Uuid,)> =
        sqlx::query_as("SELECT id FROM person_profiles WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

    if exists.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }

    // Build dynamic SET clauses
    let mut set_clauses: Vec<String> = vec!["updated_at = NOW()".to_string()];
    let mut param_idx: i32 = 1;

    let fields: Vec<(&str, Option<&str>)> = vec![
        ("first_name", data.first_name.as_deref()),
        ("last_name", data.last_name.as_deref()),
        ("gender", data.gender.as_deref()),
        ("national_id", data.national_id.as_deref()),
        ("phone", data.phone.as_deref()),
        ("address", data.address.as_deref()),
        ("city", data.city.as_deref()),
        ("district", data.district.as_deref()),
        ("emergency_contact_name", data.emergency_contact_name.as_deref()),
        ("emergency_contact_phone", data.emergency_contact_phone.as_deref()),
    ];

    // Count provided string fields for parameter indices
    let mut provided_string_fields: Vec<(&str, &str)> = Vec::new();
    for (col, val) in &fields {
        if let Some(v) = val {
            set_clauses.push(format!("{} = ${}", col, param_idx));
            provided_string_fields.push((col, v));
            param_idx += 1;
        }
    }

    let has_dob = data.date_of_birth.is_some();
    if has_dob {
        set_clauses.push(format!("date_of_birth = ${}", param_idx));
        param_idx += 1;
    }

    let sql = format!(
        "UPDATE person_profiles SET {} WHERE id = ${} RETURNING *",
        set_clauses.join(", "),
        param_idx
    );

    let mut query = sqlx::query_as::<_, PersonProfileRow>(&sql);

    for (_, val) in &provided_string_fields {
        query = query.bind(*val);
    }
    if let Some(dob) = data.date_of_birth {
        query = query.bind(dob);
    }
    query = query.bind(id);

    query.fetch_one(pool).await
}

pub async fn get_person_profile(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<PersonProfileWithUser>, sqlx::Error> {
    let row = sqlx::query_as::<_, PersonProfileUserRow>(
        "SELECT
           pp.*,
           u.id        AS u_id,
           u.email     AS u_email,
           u.name      AS u_name,
           u.phone     AS u_phone,
           u.role      AS u_role,
           u.kyc_level AS u_kyc_level,
           u.is_active AS u_is_active
         FROM person_profiles pp
         JOIN users u ON u.id = pp.user_id
         WHERE pp.id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(PersonProfileUserRow::into_profile_with_user))
}

pub async fn get_person_profile_by_user_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<PersonProfileWithUser>, sqlx::Error> {
    let row = sqlx::query_as::<_, PersonProfileUserRow>(
        "SELECT
           pp.*,
           u.id        AS u_id,
           u.email     AS u_email,
           u.name      AS u_name,
           u.phone     AS u_phone,
           u.role      AS u_role,
           u.kyc_level AS u_kyc_level,
           u.is_active AS u_is_active
         FROM person_profiles pp
         JOIN users u ON u.id = pp.user_id
         WHERE pp.user_id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(PersonProfileUserRow::into_profile_with_user))
}

pub async fn create_senior_profile(
    pool: &PgPool,
    data: &CreateSeniorProfileData,
) -> Result<SeniorProfileRow, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query_as::<_, SeniorProfileRow>(
        "INSERT INTO senior_profiles (
           id, person_id, mobility_level, cognitive_level_score,
           preferred_language, dietary_restrictions, notes, created_at, updated_at
         ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
         RETURNING *",
    )
    .bind(id)
    .bind(data.person_id)
    .bind(&data.mobility_level)
    .bind(data.cognitive_level_score)
    .bind(&data.preferred_language)
    .bind(&data.dietary_restrictions)
    .bind(&data.notes)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await
}

pub async fn update_senior_profile(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateSeniorProfileData,
) -> Result<SeniorProfileRow, sqlx::Error> {
    // Verify the profile exists
    let exists: Option<(Uuid,)> =
        sqlx::query_as("SELECT id FROM senior_profiles WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

    if exists.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }

    // Build dynamic SET clauses
    let mut set_clauses: Vec<String> = vec!["updated_at = NOW()".to_string()];
    let mut param_idx: i32 = 1;

    let string_fields: Vec<(&str, Option<&str>)> = vec![
        ("mobility_level", data.mobility_level.as_deref()),
        ("preferred_language", data.preferred_language.as_deref()),
        ("dietary_restrictions", data.dietary_restrictions.as_deref()),
        ("notes", data.notes.as_deref()),
    ];

    let mut provided_string_fields: Vec<(&str, &str)> = Vec::new();
    for (col, val) in &string_fields {
        if let Some(v) = val {
            set_clauses.push(format!("{} = ${}", col, param_idx));
            provided_string_fields.push((col, v));
            param_idx += 1;
        }
    }

    let has_score = data.cognitive_level_score.is_some();
    if has_score {
        set_clauses.push(format!("cognitive_level_score = ${}", param_idx));
        param_idx += 1;
    }

    let sql = format!(
        "UPDATE senior_profiles SET {} WHERE id = ${} RETURNING *",
        set_clauses.join(", "),
        param_idx
    );

    let mut query = sqlx::query_as::<_, SeniorProfileRow>(&sql);

    for (_, val) in &provided_string_fields {
        query = query.bind(*val);
    }
    if let Some(score) = data.cognitive_level_score {
        query = query.bind(score);
    }
    query = query.bind(id);

    query.fetch_one(pool).await
}

pub async fn get_senior_profile(
    pool: &PgPool,
    person_id: Uuid,
) -> Result<Option<SeniorProfileWithPerson>, sqlx::Error> {
    let row = sqlx::query_as::<_, SeniorProfileJoinRow>(
        "SELECT
           sp.*,
           pp.id                       AS pp_id,
           pp.user_id                  AS pp_user_id,
           pp.first_name               AS pp_first_name,
           pp.last_name                AS pp_last_name,
           pp.date_of_birth            AS pp_date_of_birth,
           pp.gender                   AS pp_gender,
           pp.national_id              AS pp_national_id,
           pp.phone                    AS pp_phone,
           pp.address                  AS pp_address,
           pp.city                     AS pp_city,
           pp.district                 AS pp_district,
           pp.emergency_contact_name   AS pp_emergency_contact_name,
           pp.emergency_contact_phone  AS pp_emergency_contact_phone,
           pp.created_at               AS pp_created_at,
           pp.updated_at               AS pp_updated_at,
           u.id                        AS u_id,
           u.email                     AS u_email,
           u.name                      AS u_name,
           u.role                      AS u_role
         FROM senior_profiles sp
         JOIN person_profiles pp ON pp.id = sp.person_id
         JOIN users u ON u.id = pp.user_id
         WHERE sp.person_id = $1",
    )
    .bind(person_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(SeniorProfileJoinRow::into_senior_with_person))
}
