// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
enum Type {
    MaintenanceCrews,
    MarketingDepartmentEmployees,
    Managers,
    LineSupervisors,
    KitchenStaff,
    AssemblyTechnicians,
}

enum Status {
    Active, 
    Terminated,
}

// * Use a struct to store the employee type and whether they are
//   still employed
struct Staff {
    position: Type,
    status: Status, 
}
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
fn check(staff: &Staff) -> Result<(), String> {
    match staff.status {
        Status::Terminated => return Err("Terminated.".to_owned()),
        _ => (),
    }

    match staff.position {
        Type::MaintenanceCrews => Ok(()),
        Type::MarketingDepartmentEmployees => Ok (()),
        Type::Managers => Ok (()),
        _ => Err("Invalid position.".to_owned()),
    }
}

fn print(staff: &Staff) -> Result <(), String> {
    let try_me = check(staff)?;
    println!("access ok");
    Ok(())
}

fn main() {
    let staff1 = Staff {
        position: Type::Managers,
        status: Status::Terminated, 
    };

    match print(&staff1) {
        Err(e) => println!("access denied: {:?}", e),
        _ => (),
    }
}
