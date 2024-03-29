use std::collections::HashMap;

pub fn get_frame(attempts: i32, won: bool) -> String {

    let main_scene = HashMap::from([   // Is this bad practice? Probably. But it's a small project and the animation itself is short.
    (0, "
        ----------------------------
        | |
        | |
        | |
        | |            
        | |           
        | |           
        | |          ______
        | |          |    |         
        "),
        (1, "
        ----------------------------
        | |
        | |
        | |
        | |            
        | |           
        | |                     O
        | |          ______    /|\\
        | |          |    |     /\\
        "),
        (2, "
        ----------------------------
        | |
        | |
        | |
        | |            
        | |          
        | |                  O
        | |          ______ /|\\
        | |          |    |  /\\
        "),
        (3, "
        ----------------------------
        | |
        | |
        | |
        | |            O
        | |           /|\\
        | |            /\\
        | |          ______
        | |          |    |         
        "),
        (4, "
        ----------------------------
        | |            |
        | |
        | |
        | |            O
        | |           /|\\
        | |            /\\
        | |          ______
        | |          |    |         
        "),
        (5,  "
        ----------------------------
        | |            |
        | |            |
        | |
        | |            O
        | |           /|\\
        | |            /\\
        | |          ______
        | |          |    |         
        "),
        (6, "
        ----------------------------
        | |            |
        | |            |
        | |            |
        | |            O
        | |           /|\\
        | |            /\\
        | |          ______
        | |          |    |         
        "),
        (7, "
        ----------------------------
        | |            |
        | |            |
        | |            |
        | |            O
        | |           /|\\
        | |            /\\
        | |         
        | |                
        "),
        (8, "
        ----------------------------
        | |            |
        | |            |
        | |            |
        | |            x
        | |           /|\\
        | |            /\\
        | |         
        | |                
        "),
        (9, "
        ----------------------------
        | |            
        | |            
        | |            
        | |           
        | |           
        | |            O
        | |           /|\\
        | |            /\\
        "),
    ]);

    if won {
        return main_scene.get(&9).unwrap().to_string();
    }

    return main_scene.get(&attempts).unwrap().to_string();
}