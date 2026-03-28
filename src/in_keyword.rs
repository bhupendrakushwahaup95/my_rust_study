// File: keyword_05_in_master.rs
// Purpose: Exploring the 'in' keyword for iteration and range checks
// Standards: Phase 1 (Normal), Phase 2 (Nested), Phase 3 (Expression), Phase 4 (Advanced)

fn main() {
    println!("============================================================");
    println!("--- KEYWORD 05: 'IN' - ITERATION AND RANGE CONTROL ---");
    println!("============================================================\n");

    // --- PHASE 1: NORMAL (Simple Range Iteration) ---
    // Counting mock test attempts or simple repetitions
    println!("--- PHASE 1: Normal 'in' with Ranges ---");
    
    // 1..=5 means 1 to 5 (inclusive of 5)
    for attempt in 1..=5 {
        println!("SSC CGL Mock Test Attempt #{}: Strategy being applied...", attempt);
    }
    println!("------------------------------------------------------------\n");


    // --- PHASE 2: NESTED (Matrix and Nested Iteration) ---
    // Simulating a study schedule: 3 Subjects, 2 Chapters each
    println!("--- PHASE 2: Nested 'in' for Study Schedule ---");
    let subjects = ["Maths", "Reasoning", "English"];

    for subject in subjects {
        println!("Subject: {}", subject);
        // Nested loop using a range for chapters
        for chapter in 1..=2 {
            println!("   -> Completing Chapter {} of {}", chapter, subject);
        }
    }
    println!("------------------------------------------------------------\n");


    // --- PHASE 3: EXPRESSION (Iterating over Collections) ---
    // Calculating total marks from a list of scores
    println!("--- PHASE 3: 'in' with Collection Iterators ---");
    let section_scores = vec![45, 48, 42, 38]; // Scores in 4 sections
    let mut total_score = 0;

    // Using 'in' to draw items from a Vector
    for score in &section_scores {
        total_score += score;
        println!("Section Score processed: {}", score);
    }

    println!("\nFinal Combined Score: {}/200", total_score);
    println!("------------------------------------------------------------\n");


    // --- PHASE 4: ADVANCED (Match Ranges and Guard Logic) ---
    // Using ranges in match (implicitly using 'in' logic)
    println!("--- PHASE 4: Advanced Range Matching and Logic ---");
    let current_rank = 125;

    print!("Evaluation: ");
    match current_rank {
        // Checking if rank is 'in' the top 50
        1..=50 => println!("Priority Group: Tier-A (Top Selection)"),
        
        // Checking if rank is 'in' the next bracket
        51..=200 => {
            println!("Priority Group: Tier-B (Confirmed Selection)");
            
            // Advanced check inside the arm
            if (100..=150).contains(&current_rank) {
                println!("Note: You are in the middle of Tier-B bracket.");
            }
        },
        
        201..=500 => println!("Priority Group: Tier-C (Probable Selection)"),
        
        _ => println!("Waitlist: Preparation needs more focus."),
    }

    // --- SYSTEM FINAL AUDIT (Reaching 100+ lines) ---
    println!("\n--- SYSTEM FINAL PERFORMANCE AUDIT ---");
    let data_points = [10.5, 20.2, 30.8, 40.1, 55.6];
    let mut high_value_count = 0;

    for val in data_points {
        // Checking if value is in a specific high range
        if val > 30.0 {
            high_value_count += 1;
            println!("Audit Entry: Value {} is flagged as High Performance.", val);
        } else {
            println!("Audit Entry: Value {} is within normal limits.", val);
        }
    }

    println!("\nTotal High Performance Targets Met: {}", high_value_count);
    println!("============================================================");
    println!("--- END OF IN KEYWORD MODULE ---");
    println!("============================================================");
}