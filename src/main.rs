use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Represents a position in 3D space.
#[derive(Clone, Copy, Debug)]
pub struct Position {
    /// The x-coordinate of the position.
    x: f64,
    /// The y-coordinate of the position.
    y: f64,
    /// The z-coordinate of the position.
    z: f64,
}

impl Position {
    /// Creates a new Position instance with the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate.
    /// * `y` - The y-coordinate.
    /// * `z` - The z-coordinate.
    ///
    /// # Returns
    ///
    /// A new `Position` instance.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Position { x, y, z }
    }

    /// Moves the position linearly towards the target position.
    ///
    /// Stops when the position is within a small distance of the stop coordinates.
    ///
    /// # Arguments
    ///
    /// * `target` - The target position to move towards.
    /// * `stop_coordinates` - The stop coordinates indicating when to stop the movement.
    pub fn linear_move(&mut self, target: &Position, stop_coordinates: &Position) {
        let dx = (target.x - self.x) / 5.0;
        let dy = (target.y - self.y) / 5.0;
        let dz = (target.z - self.z) / 5.0;
        for _ in 1..=5 {
            self.x += dx;
            self.y += dy;
            self.z += dz;
            println!("{:.2}, {:.2}, {:.2}", self.x, self.y, self.z);
            let distance = ((self.x - stop_coordinates.x).powi(2) + (self.y - stop_coordinates.y).powi(2) + (self.z - stop_coordinates.z).powi(2)).sqrt();
            if distance < 0.001 { // Adjust the threshold as needed
                println!("Reached stop coordinates: {:.2}, {:.2}, {:.2}", self.x, self.y, self.z);
                break;
            }
        }
    }

    /// Moves the position along a circular path.
    ///
    /// Stops when the position is within a small distance of the stop coordinates.
    ///
    /// # Arguments
    ///
    /// * `center` - The center position of the circular path.
    /// * `radius` - The radius of the circular path.
    /// * `direction` - The direction of movement ("CW" for clockwise, "CCW" for counterclockwise).
    /// * `stop_coordinates_cw` - The stop coordinates for clockwise movement.
    /// * `stop_coordinates_ccw` - The stop coordinates for counterclockwise movement.
    pub fn circular_move(
        &mut self,
        center: &Position,
        radius: f64,
        direction: &str,
        stop_coordinates_cw: &Position,
        stop_coordinates_ccw: &Position,
    ) {
        let theta_step: f64 =  5_f64.to_radians(); // Angle increment in radians
        let theta_step = if direction == "CW" { -theta_step } else { theta_step }; // Adjust step based on direction
        let num_steps = (2.0 * std::f64::consts::PI * radius / theta_step.abs()).ceil() as usize;
        for i in 0..num_steps {
            let theta = (i as f64) * theta_step;
            self.x = center.x + radius * theta.cos();
            self.y = center.y + radius * theta.sin();
            self.z = center.z;
            println!("{:.2}, {:.2}, {:.2}", self.x, self.y, self.z);
            let stop_coordinates = if direction == "CW" {
                stop_coordinates_cw
            } else {
                stop_coordinates_ccw
            };
            let distance = ((self.x - stop_coordinates.x).powi(2) + (self.y - stop_coordinates.y).powi(2) + (self.z - stop_coordinates.z).powi(2)).sqrt();
            if distance < 0.001 { // Adjust the threshold as needed
                println!("Reached stop coordinates: {:.2}, {:.2}, {:.2}", self.x, self.y, self.z);
                break;
            }
        }
    }
}

// Main function and tests...


fn main() -> io::Result<()> {
    let file_path = "code.cmmd"; 
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();

    let mut current_position = Position::new(0.0, 0.0, 0.0);
    let mut last_position = Position::new(0.0, 0.0, 0.0);
    let mut stop_coordinates_cw = Position::new(0.0, 0.0, 0.0);
    let mut stop_coordinates_ccw = Position::new(0.0, 0.0, 0.0);

    for line in lines {
        if let Ok(command) = line {
            println!("Command read from file: {}", command); // Debugging output
            let parts: Vec<&str> = command.split_whitespace().collect();
            println!("Parts: {:?}", parts); // Debugging output
            match parts[0] {
                "LIN" => {
                    if parts.len() == 4 {
                        let coordinates: Vec<&str> = parts[1..].iter().map(|s| &s[1..]).collect();
                        if let (Ok(x), Ok(y), Ok(z)) = (
                            coordinates[0].parse::<f64>(),
                            coordinates[1].parse::<f64>(),
                            coordinates[2].parse::<f64>(),
                        ) {
                            let target_position = Position::new(x, y, z);
                            last_position = current_position;
                            current_position.linear_move(&target_position, &stop_coordinates_cw);
                        } else {
                            eprintln!("Invalid coordinates: {:?}", coordinates);
                        }
                    } else {
                        eprintln!("Invalid input format: {}", command);
                    }
                }
                "CW" => {
                    if parts.len() == 7 {
                        let x = parts[1][1..].parse::<f64>().unwrap();
                        let y = parts[2][1..].parse::<f64>().unwrap();
                        let z = parts[3][1..].parse::<f64>().unwrap();
                        let center_x = parts[4][1..].parse::<f64>().unwrap();
                        let center_y = parts[5][1..].parse::<f64>().unwrap();
                        let center_z = parts[6][1..].parse::<f64>().unwrap();
                        stop_coordinates_cw = Position::new(x, y, z);
                        let radius = ((x - center_x).powi(2) + (y - center_y).powi(2) + (z - center_z).powi(2)).sqrt();
                        current_position.circular_move(&Position::new(center_x, center_y, center_z), radius, "CW", &stop_coordinates_cw, &stop_coordinates_ccw);
                    } else {
                        eprintln!("Invalid input format: {}", command);
                    }
                }
                "CCW" => {
                    if parts.len() == 7 {
                        let x = parts[1][1..].parse::<f64>().unwrap();
                        let y = parts[2][1..].parse::<f64>().unwrap();
                        let z = parts[3][1..].parse::<f64>().unwrap();
                        let center_x = parts[4][1..].parse::<f64>().unwrap();
                        let center_y = parts[5][1..].parse::<f64>().unwrap();
                        let center_z = parts[6][1..].parse::<f64>().unwrap();
                        stop_coordinates_ccw = Position::new(x, y, z);
                        let radius = ((x - center_x).powi(2) + (y - center_y).powi(2) + (z - center_z).powi(2)).sqrt();
                        current_position.circular_move(&Position::new(center_x, center_y, center_z), radius, "CCW", &stop_coordinates_cw, &stop_coordinates_ccw);
                    } else {
                        eprintln!("Invalid input format: {}", command);
                    }
                }
                _ => {
                    eprintln!("Invalid command: {}", command);
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_move() {
        // Test cases for linear_move function
        let mut current_position = Position::new(0.0, 0.0, 0.0);
        let target_position = Position::new(5.0, 5.0, 5.0);
        let stop_coordinates = Position::new(5.0, 5.0, 5.0);
        current_position.linear_move(&target_position, &stop_coordinates);

        assert_eq!(current_position.x, stop_coordinates.x);
        assert_eq!(current_position.y, stop_coordinates.y);
        assert_eq!(current_position.z, stop_coordinates.z);
        // Add assertions to verify the correctness of linear_move function
    }

    #[test]
    
    fn test_circular_move() {
        // Test cases for circular_move function
        let mut current_position = Position::new(0.0, 0.0, 0.0);
        let center = Position::new(1.25, 1.25, 5.0);
        let radius = ((7.5 - center.x).powi(2) + (7.5 - center.y).powi(2) + (5.0 - center.z).powi(2)).sqrt(); // Calculate radius dynamically
        let stop_coordinates_cw = Position::new(7.5, 7.5, 5.0); // Adjust as needed
        let stop_coordinates_ccw = Position::new(5.0, 5.0, 5.0); // Adjust as needed
        current_position.circular_move(&center, radius, "CW", &stop_coordinates_cw, &stop_coordinates_ccw);
    
        // Allow for a small tolerance (e.g., 0.001) when comparing floating-point numbers
        let tolerance = 0.001;
        assert!((current_position.x - stop_coordinates_cw.x).abs() < tolerance);
        assert!((current_position.y - stop_coordinates_cw.y).abs() < tolerance);
        assert!((current_position.z - stop_coordinates_cw.z).abs() < tolerance);
}

}
