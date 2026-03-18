export interface GeocodingResult {
  readonly latitude: number;
  readonly longitude: number;
  readonly address: string;
  readonly displayName: string;
}

export interface DistanceResult {
  readonly distanceKm: number;
  readonly durationMinutes: number;
}

export interface MapProvider {
  geocode(address: string): Promise<GeocodingResult>;
  reverseGeocode(lat: number, lng: number): Promise<GeocodingResult>;
  calculateDistance(
    from: { lat: number; lng: number },
    to: { lat: number; lng: number },
  ): Promise<DistanceResult>;
}
