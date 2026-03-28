    // File: if_keyword.rs
    // Purpose: Comprehensive demonstration of the 'if' keyword in Rust
    // Standards: Phase 1 (Normal), Phase 2 (Nested), Phase 3 (Expression), Phase 4 (Advanced)

fn main(){
    println!("======================================================================");
    println!("--- KEYWORD 1: 'IF' - MULTI-PHASE LOGIC DEMONSTRATION ---");
    println!("======================================================================");

    // --- Phase 1: Normal (Basic Decision Making)---
    // Checking if a candidate meets the basic age criteria for SSC CGL 
    println!("--- Phase 1: Normal Usage ----");
    let candidate_age: u32 = 24;
    let minimum_required_age: u32 = 18;

    if candidate_age >= minimum_required_age {
        println!("Status: Candidate meets the minimum age requirement.");
        println!("Action: proceed to document verification.");
    }
    // Note: If candition is false, nothing happens here as there is no else.
    println!("------------------------------------------------------\n");

    //--- Phase 2: Nested (Complex Layered Conditions) ---
    // Checking Tier-1 qualification and then specific subject cut-off
    println!("--- Pase 2: Nested Logic ---");
    let has_cleared_tier_1: bool = true;
    let mathematics_marks: i32 = 45; // out of 50 
    let maths_cutoff: i32 = 40;

    if has_cleared_tier_1{
        println!("Step 1: Tier-1 Clearance Verified.");
        
        // Nested IF to check subject-specific performance
        if mathematics_marks >= maths_cutoff {
            println!("Step 2: Subject cutoff cleared with {} marks.", mathematics_marks);
            println!("Final Status: Eligible for Tier-2 Mains Exam.");
        }else{
            println!("Step 2: Subject cutoff NOT cleared. Better luck next time.");
        }
    }else {
        println!("Status: Candidate did not clear Tier-1");
    }
    println!("-----------------------------------------------------------------\n");

    // --- PHASE 3: EXPRESSION (Returning Values directly) ---
    // Using 'if' as an expression to assign a grade to a variable
    println!("--- PHASE 3: Conditional Expression ---");
    let total_marks_percentage: f64 = 88.5;

    // In Rust, 'if' returns a value that can be stored in a variable.
    // Ensure all branches return the same data type (&str in this case).
    let performance_grade = if total_marks_percentage >= 90.0 {
        "Excellent (A+)"
    } else if total_marks_percentage >= 75.0 {
        "Very Good (A)"
    } else if total_marks_percentage >= 60.0 {
        "Good (B)"
    } else {
        "Average (C)"
    };

    println!("Exam Score: {}%", total_marks_percentage);
    println!("Assigned Grade: {}", performance_grade);
    println!("------------------------------------------------------------\n");


    // --- PHASE 4: ADVANCED (The 'if let' Pattern) ---
    // Handling Optional data (like a Rank that might or might not exist)
    println!("--- PHASE 4: Advanced Pattern Matching (if let) ---");
    let mock_test_rank: Option<u32> = Some(15); // Simulating a top 20 rank

    // 'if let' is a concise way to handle values wrapped in Option or Result
    if let Some(actual_rank) = mock_test_rank {
        println!("Success: Rank data retrieved.");
        
        // Combining with logic to check if it's a Top 100 rank
        if actual_rank <= 100 {
            println!("Analysis: Rank {} is highly competitive for CGL.", actual_rank);
        }
    } else {
        println!("Error: Rank data is currently unavailable in the database.");
    }

    // --- SYSTEM FINAL CHECK (Extended Logic for Line Count) ---
    println!("\n--- SYSTEM FINAL AUDIT ---");
    let is_system_ready: bool = true;
    let database_connected: bool = true;

    if is_system_ready && database_connected {
        println!("Audit Status: All conditions met. System is operational.");
    } else {
        println!("Audit Status: Critical failure in system conditions.");
    }

    println!("============================================================");
    println!("--- END OF IF KEYWORD MODULE ---");
    println!("============================================================");
}
