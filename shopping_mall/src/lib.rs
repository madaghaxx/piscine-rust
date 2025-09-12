mod mall;
pub use mall::*;
pub use std::collections::HashMap;

// returns the Store with the most square meters
pub fn biggest_store(mall: &Mall) -> Store {
    let mut best: Option<&Store> = None;
    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            best = match best {
                Some(current) if current.square_meters >= store.square_meters => Some(current),
                _ => Some(store),
            };
        }
    }
    best.cloned().expect("mall has no stores")
}

// returns the Employee(s) with the highest salary
pub fn highest_paid_employee(mall: &Mall) -> Vec<Employee> {
    let mut best_salary: Option<f64> = None;
    let mut best_employees: Vec<Employee> = Vec::new();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for emp in store.employees.values() {
                match best_salary {
                    None => {
                        best_salary = Some(emp.salary);
                        best_employees.clear();
                        best_employees.push(*emp);
                    }
                    Some(s) if emp.salary > s => {
                        best_salary = Some(emp.salary);
                        best_employees.clear();
                        best_employees.push(*emp);
                    }
                    Some(s) if emp.salary == s => {
                        best_employees.push(*emp);
                    }
                    _ => {}
                }
            }
        }
    }

    if best_salary.is_some() {
        best_employees
    } else {
        panic!("mall has no employees");
    }
}

// returns the number of employees and guards
pub fn nbr_of_employees(mall: &Mall) -> usize {
    let guard_count = mall.guards.len();
    let employee_count: usize = mall.floors
        .values()
        .map(|f|
            f.stores
                .values()
                .map(|s| s.employees.len())
                .sum::<usize>()
        )
        .sum();
    guard_count + employee_count
}

// ensure at least 1 guard per 200 total square meters of floor size
pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let total_floor_size: u64 = mall.floors
        .values()
        .map(|f| f.size_limit)
        .sum();

    let required_guards: usize = if total_floor_size == 0 {
        0
    } else {
        ((total_floor_size + 199) / 200) as usize
    };

    let current = mall.guards.len();
    if current >= required_guards {
        return;
    }

    let mut need = required_guards - current;
    for (name, guard) in guards.into_iter() {
        if need == 0 {
            break;
        }
        if !mall.guards.contains_key(&name) {
            mall.guards.insert(name, guard);
            need -= 1;
        }
    }
}

// adjust salaries by 10% based on working hours (>=10 hours => raise, else cut)
pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for emp in store.employees.values_mut() {
                let (from, to) = emp.working_hours;
                let hours = to.saturating_sub(from);
                let delta = emp.salary * 0.1;
                if hours >= 10 {
                    emp.raise(delta);
                } else {
                    emp.cut(delta);
                }
            }
        }
    }
}
