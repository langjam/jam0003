use std::collections::HashMap;

use crate::prelude::*;

/// Parse the file into a map of points to characters
pub fn parse(file: &String, folder: String) -> Program {
    // Make a hashmap of every character
    let mut map: FileMap = HashMap::new();

    // Create the bounds of the map
    let mut bounds = Bounds::new();

    // Iterate over the file
    file.lines().enumerate().for_each(|(i, line)| {
        // Iterate over the line
        line.chars().enumerate().for_each(|(j, c)| {
            let syntax = c.into();

            // Add the character to the hashmap
            map.insert(
                Point {
                    x: j as i32,
                    y: i as i32,
                },
                Place::new(syntax),
            );

            // Only update the bounds if the character is not floor
            if syntax != Syntax::Floor {
                bounds.update(j as i32, i as i32);
            }
        });
    });

    Program::new(map, bounds, folder)
}
