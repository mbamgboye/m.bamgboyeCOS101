fn main() {
    // 1. Define the table as a Vector of Tuples
    // Format: (APS_Level, Office_Admin, Academic, Lawyer, Teacher)
    let aps_table: Vec<(&str, &str, &str, &str, &str)> = vec![
        ("APS 1-2", "Intern", "-", "Paralegal", "Placement"),
        ("APS 3-5", "Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"),
        ("APS 5-8", "Senior Administrator", "PhD Candidate", "Associate", "Senior Teacher"),
        ("EL1 8-10", "Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"),
        ("EL2 10-13", "Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"),
        ("SES", "CEO", "Dean", "Partner", "Principal"),
    ];

    // 2. Test the checker with an example: an Associate Lawyer
    let profession = "Lawyer";
    let role = "Associate";

    match check_aps_level(&aps_table, profession, role) {
        Some(level) => println!("Match found! An {} with role '{}' holds position: {}", profession, role, level),
        None => println!("Role not found in the table."),
    }
}

// 3. Validator Function
fn check_aps_level<'a>(
    table: &[(&'a str, &'a str, &'a str, &'a str, &'a str)], 
    profession: &str, 
    role: &str
) -> Option<&'a str> {
    
    // Determine column index based on profession
    let col_index = match profession.to_lowercase().as_str() {
        "office admin" => 1,
        "academic" => 2,
        "lawyer" => 3,
        "teacher" => 4,
        _ => return None, // Invalid profession
    };

    // Iterate through the vector of tuples
    for row in table {
        let current_role = match col_index {
            1 => row.1,
            2 => row.2,
            3 => row.3,
            4 => row.4,
            _ => "",
        };

        // Check if the role matches case-insensitively
        if current_role.eq_ignore_ascii_deg(role) || current_role.eq_ignore_ascii_case(role) {
            return Some(row.0); // Return the APS level (first element of the tuple)
        }
    }

    None
}