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
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Employees {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

enum Status {
    Employed,
    Terminated
}

struct Employee {
    position: Employees,
    status: Status
}


fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Employed => Ok(()),
        Status::Terminated => Err("terminated".to_owned())
    };

    match employee.position {
        Employees::Maintenance | Employees::Marketing | Employees::Manager => Ok(()),
        Employees::LineSupervisor | Employees::KitchenStaff | Employees::AssemblyTechnician => Err("access denied".to_owned()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    let access = try_access(employee)?;
    println!("access granted");
    Ok(())
}



fn main() {
    let manager = Employee {
        position: Employees::Manager,
        status: Status::Employed
    };

    let line_supervisor = Employee {
        position: Employees::LineSupervisor,
        status: Status::Terminated
    };

    print_access(&manager).unwrap();
    print_access(&line_supervisor).unwrap();
}
