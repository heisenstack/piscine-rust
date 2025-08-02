pub mod mall;
pub use mall::{Employee, Floor, Guard, Mall, Store};
pub use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut largest_store: Store = Store::new(HashMap::<String, Employee>::new(), 0);
    let mut largest_store_name: String = String::new();

    for floor in mall.floors.values() {
        for (store_name, store) in &floor.stores {
            if store.square_meters > largest_store.square_meters {
                largest_store_name = store_name.clone();
                largest_store = store.clone();
            }
        }
    }

    (largest_store_name, largest_store)
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, Employee)> {
    let mut top_earners: Vec<(&str, Employee)> = Vec::new();
    let mut max_salary: f64 = 0.0;

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (employee_name, employee) in &store.employees {
                if employee.salary > max_salary {
                    top_earners.clear();
                    top_earners.push((employee_name, *employee));
                    max_salary = employee.salary;
                } else if (employee.salary - max_salary).abs() < f64::EPSILON {
                    top_earners.push((employee_name, *employee));
                }
            }
        }
    }

    top_earners
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut total_employees = mall.guards.len();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            total_employees += store.employees.len();
        }
    }

    total_employees
}
pub fn check_for_securities(m: &mut Mall, guards: Vec<(String, Guard)>) {
    let temp = m.guards.len();
    let mut nbr: f64 = 0.0;
    for (_, floor_data) in &m.floors {
        nbr += (floor_data.size_limit as f64 / 200.0).round()
    }
    // println!("{:?}", nbr);
    for i in 0..(nbr as usize - temp - 1) {
        m.hire_guard(&guards[i].0, guards[i].1);
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let work_hours = employee.working_hours.1 - employee.working_hours.0;
                let adjustment = employee.salary * 0.1;

                if work_hours < 10 {
                    employee.cut(adjustment);
                } else {
                    employee.raise(adjustment);
                }
            }
        }
    }
}
