import type { DistanceResult, GeocodingResult, MapProvider } from './types.js';

// Default coordinates: Seoul City Hall
const SEOUL_DEFAULT_LAT = 37.5665;
const SEOUL_DEFAULT_LNG = 126.978;

// Well-known Seoul district coordinates used for geocoding mock responses
const DISTRICT_GEOCODES: Record<string, GeocodingResult> = {
  강남구: {
    latitude: 37.5172,
    longitude: 127.0473,
    address: '서울특별시 강남구',
    displayName: '강남구, 서울특별시',
  },
  종로구: {
    latitude: 37.5735,
    longitude: 126.979,
    address: '서울특별시 종로구',
    displayName: '종로구, 서울특별시',
  },
  마포구: {
    latitude: 37.5663,
    longitude: 126.9019,
    address: '서울특별시 마포구',
    displayName: '마포구, 서울특별시',
  },
  송파구: {
    latitude: 37.5145,
    longitude: 127.1059,
    address: '서울특별시 송파구',
    displayName: '송파구, 서울특별시',
  },
  서대문구: {
    latitude: 37.5791,
    longitude: 126.9368,
    address: '서울특별시 서대문구',
    displayName: '서대문구, 서울특별시',
  },
};

const SEOUL_DEFAULT_GEOCODE: GeocodingResult = {
  latitude: SEOUL_DEFAULT_LAT,
  longitude: SEOUL_DEFAULT_LNG,
  address: '서울특별시 중구 태평로1가 31',
  displayName: '서울시청, 중구, 서울특별시',
};

/**
 * Mock map provider for development and testing.
 * Returns Seoul-area coordinates and plausible mock distances.
 */
export class MockMapProvider implements MapProvider {
  async geocode(address: string): Promise<GeocodingResult> {
    for (const [district, result] of Object.entries(DISTRICT_GEOCODES)) {
      if (address.includes(district)) {
        return result;
      }
    }
    return { ...SEOUL_DEFAULT_GEOCODE, address, displayName: address };
  }

  async reverseGeocode(lat: number, lng: number): Promise<GeocodingResult> {
    // Find nearest known district by simple Euclidean proximity
    let nearest: GeocodingResult = SEOUL_DEFAULT_GEOCODE;
    let minDist = Infinity;

    for (const result of Object.values(DISTRICT_GEOCODES)) {
      const dist = euclideanDistance(lat, lng, result.latitude, result.longitude);
      if (dist < minDist) {
        minDist = dist;
        nearest = result;
      }
    }

    return nearest;
  }

  async calculateDistance(
    from: { lat: number; lng: number },
    to: { lat: number; lng: number },
  ): Promise<DistanceResult> {
    const straightLineKm = haversineKm(from.lat, from.lng, to.lat, to.lng);

    // Apply a road-network factor of ~1.3 for urban Seoul
    const distanceKm = parseFloat((straightLineKm * 1.3).toFixed(2));

    // Estimate driving time: average 30 km/h in urban Seoul
    const durationMinutes = Math.round((distanceKm / 30) * 60);

    return { distanceKm, durationMinutes };
  }
}

function euclideanDistance(lat1: number, lng1: number, lat2: number, lng2: number): number {
  return Math.sqrt((lat1 - lat2) ** 2 + (lng1 - lng2) ** 2);
}

function haversineKm(lat1: number, lng1: number, lat2: number, lng2: number): number {
  const R = 6371; // Earth radius in km
  const dLat = toRad(lat2 - lat1);
  const dLng = toRad(lng2 - lng1);
  const a =
    Math.sin(dLat / 2) ** 2 +
    Math.cos(toRad(lat1)) * Math.cos(toRad(lat2)) * Math.sin(dLng / 2) ** 2;
  return R * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
}

function toRad(deg: number): number {
  return (deg * Math.PI) / 180;
}
