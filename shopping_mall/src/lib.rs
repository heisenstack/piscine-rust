use std::collections::HashMap;
pub mod mall;
pub use mall::{Employee, Floor, Guard, Mall, Store};

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut biggest = None;
    let mut max_size = 0;

    for (_floor_name, floor) in &mall.floors {
        for (store_name, store) in &floor.stores {
            if store.square_meters > max_size {
                max_size = store.square_meters;
                biggest = Some((store_name.clone(), store.clone()));
            }
        }
    }

    biggest.unwrap_or_else(|| {
        (
            "No stores".to_string(),
            Store {
                employees: HashMap::new(),
                square_meters: 0,
            },
        )
    })
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(String, Employee)> {
    let mut max_salary = 0.0;
    let mut highest_paid = Vec::new();

    for (_floor_name, floor) in &mall.floors {
        for (_store_name, store) in &floor.stores {
            for (emp_name, employee) in &store.employees {
                if (employee.salary - max_salary).abs() < f64::EPSILON {
                    highest_paid.push((emp_name.clone(), employee.clone()));
                } else if employee.salary > max_salary {
                    max_salary = employee.salary;
                    highest_paid.clear();
                    highest_paid.push((emp_name.clone(), employee.clone()));
                }
            }
        }
    }

    highest_paid
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut count = mall.guards.len();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            count += store.employees.len();
        }
    }

    count
}
pub fn check_for_securities(mall: &mut Mall, guards: Vec<(String, Guard)>) {
    let mut mall_area = 0;

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            mall_area += store.square_meters;
        }
    }

    let required_guards = (mall_area + 199) / 200;
    let current_guards = mall.guards.len() as u64;

    if current_guards < required_guards {
        let needed = (required_guards - current_guards) as usize;
        for (name, guard) in guards.into_iter().take(needed) {
            mall.guards.insert(name, guard);
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                if employee.working_hours.1 - employee.working_hours.0 > 10 {
                    employee.salary *= 1.1;
                } else {
                    employee.salary *= 0.9;
                }
            }
        }
    }
}
