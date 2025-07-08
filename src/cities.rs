pub struct City {
    pub name: String,
    pub population: u64,
    pub geo: (f64, f64), // longitude and latitude
}

impl City {
    pub fn new(name: &str, population: u64, longitude: f64, latitude: f64) -> Self {
        City {
            name: name.to_string(),
            population,
            geo: (longitude, latitude),
        }
    }
}

pub fn get_hanseatic_cities() -> Vec<City> {
    vec![
        // Major Baltic Sea Ports
        City::new("Lübeck", 25000, 10.6865, 53.8655),
        City::new("Hamburg", 22000, 9.9937, 53.5511),
        City::new("Bremen", 18000, 8.8017, 53.0793),
        City::new("Rostock", 12000, 12.1400, 54.0887),
        City::new("Wismar", 8000, 11.4669, 53.8928),
        City::new("Stralsund", 9000, 13.0813, 54.3093),
        City::new("Greifswald", 7000, 13.3888, 54.0865),
        City::new("Stettin", 15000, 14.5528, 53.4289),
        City::new("Danzig", 20000, 18.6466, 54.3520),
        City::new("Königsberg", 10000, 20.4522, 54.7065),
        City::new("Riga", 12000, 24.1052, 56.9496),
        City::new("Reval", 8000, 24.7536, 59.4370),
        City::new("Dorpat", 6000, 26.7290, 58.3776),
        City::new("Narva", 5000, 28.1907, 59.3773),
        City::new("Visby", 7000, 18.2948, 57.6348),
        City::new("Stockholm", 8000, 18.0686, 59.3293),
        City::new("Kalmar", 6000, 16.3516, 56.6634),
        City::new("Bergen", 10000, 5.3221, 60.3913),
        City::new("Oslo", 8000, 10.7522, 59.9139),
        
        // North Sea and Atlantic Ports
        City::new("Bruges", 45000, 3.2247, 51.2093),
        City::new("Antwerp", 35000, 4.4025, 51.2194),
        City::new("Amsterdam", 12000, 4.9041, 52.3676),
        City::new("Kampen", 8000, 5.9111, 52.5555),
        City::new("Deventer", 7000, 6.1639, 52.2551),
        City::new("Zutphen", 6000, 6.2003, 52.1394),
        City::new("Zwolle", 7000, 6.0919, 52.5168),
        City::new("Groningen", 9000, 6.5665, 53.2194),
        City::new("Emden", 5000, 7.2061, 53.3594),
        City::new("Stade", 6000, 9.4777, 53.5939),
        City::new("Buxtehude", 4000, 9.6947, 53.4716),
        City::new("Lüneburg", 10000, 10.4041, 53.2493),
        
        // River Access to Sea
        City::new("Cologne", 40000, 6.9603, 50.9375),
        City::new("Dortmund", 8000, 7.4653, 51.5136),
        City::new("Münster", 6000, 7.6261, 51.9607),
        City::new("Osnabrück", 5000, 8.0472, 52.2799),
        City::new("Magdeburg", 14000, 11.6276, 52.1205),
        City::new("Brandenburg", 7000, 12.5584, 52.4125),
        City::new("Frankfurt an der Oder", 8000, 14.5504, 52.3476),
        City::new("Thorn", 9000, 18.6031, 53.0138),
        City::new("Cracow", 18000, 19.9450, 50.0647),
        
        // Additional Important Maritime-Connected Cities
        City::new("Novgorod", 30000, 31.2985, 58.5244),
        City::new("Pskov", 15000, 28.3496, 57.8136),
        City::new("Smolensk", 12000, 32.0401, 54.7818),
        City::new("Polotsk", 8000, 28.7855, 55.4857),
        City::new("Dinant", 6000, 4.9120, 50.2609),
        City::new("Stavanger", 5000, 5.7331, 58.9700),
        City::new("Trondheim", 4000, 10.3951, 63.4305),
        City::new("Åbo", 6000, 22.2666, 60.4518),
        City::new("Helsinki", 3000, 24.9384, 60.1699),
        City::new("Pernau", 4000, 24.5040, 58.3859),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_city_creation() {
        let lubeck = City::new("Lübeck", 25000, 10.6865, 53.8655);
        assert_eq!(lubeck.name, "Lübeck");
        assert_eq!(lubeck.population, 25000);
        assert_eq!(lubeck.geo.0, 10.6865);
        assert_eq!(lubeck.geo.1, 53.8655);
    }

    #[test]
    fn test_hanseatic_cities_count() {
        let cities = get_hanseatic_cities();
        assert_eq!(cities.len(), 50);
    }

    #[test]
    fn test_cities_have_valid_coordinates() {
        let cities = get_hanseatic_cities();
        for city in cities {
            // Longitude should be between -180 and 180
            assert!(city.geo.0 >= -180.0 && city.geo.0 <= 180.0, 
                    "Invalid longitude for {}: {}", city.name, city.geo.0);
            // Latitude should be between -90 and 90
            assert!(city.geo.1 >= -90.0 && city.geo.1 <= 90.0, 
                    "Invalid latitude for {}: {}", city.name, city.geo.1);
        }
    }
} 