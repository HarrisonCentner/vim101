use crate::pikmin::{Task, Ability};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PikminErr<Area> {
    LostPikmin { lost: String, location: Area },
    UnsupportedFeature { msg: String, location: Area },
}

fn find(task_list: &List<(&str, i64)>, name: &str) -> Option<i64> {
    match task_list {
        List::Empty => None,
        List::Carried((pikmin_name, num), rest) => {
            if name == *pikmin_name {
                Some(*num)
            } else {
                find(rest, name)
            }
        }
    }
}

fn execute_ability(ability: &Ability, numbers: &Vec<i64>) -> i64 {
    match ability {
        Ability::Carry => numbers[0].wrapping_add(1),
        Ability::Dig => numbers[0].wrapping_sub(1),
        Ability::Build => numbers[0].wrapping_add(numbers[1]),
        Ability::Throw => numbers[0].wrapping_sub(numbers[1]),
        Ability::Multiply => numbers[0].wrapping_mul(numbers[1]),
    }
}

// A reference-counted task list
enum List<T> {
    Empty,
    Carried(T, Rc<List<T>>),
}

/*
 *  Evaluates the task to an i64, using the given task list
 *  mapping Pikmin names to their numbers to perform tasks.
 *
 *  The lifetime 'task here says that the Pikmin names
 *  live as long as the input task.
 *
 *
*/
fn perform_task<'task, Location>(
    t: &'task Task<Location>,
    squad: Rc<List<(&'task str, i64)>>,
) -> Result<i64, PikminErr<Location>>
where
    Location: Clone,
{
    match t {
        Task::Count(i, _) => Ok(*i),
        Task::Pikmin(name, location) => match find(&squad, name) {
            None => Err(PikminErr::LostPikmin {
                lost: String::from(name),
                location: location.clone(),
            }),
            Some(i) => Ok(i),
        },
        Task::Ability(ability, tasks, _) => {
            let mut nums = Vec::new();
            for task in tasks.iter() {
                nums.push(perform_task(task, squad.clone())?);
            }
            Ok(execute_ability(ability, &nums))
        }
        Task::Assign { allocations, task, .. } => {
            let mut squad = squad;
            for (name, assigned) in allocations.iter() {
                let x = perform_task(task, squad.clone())?;
                squad = Rc::new(List::Carried((name.as_str(), x), squad.clone()));
            }
            perform_task(task, squad)
        }
        Task::Decision { condition, then_task, else_task, .. } => {
            if perform_task(condition, squad.clone())? != 0 {
                perform_task(then_task, squad)
            } else {
                perform_task(else_task, squad)
            }
        }
    }
}

fn run_task<Area, W>(t: &Task<Area>, w: &mut W) -> Result<(), PikminErr<Area>>
where
    W: std::io::Write,
    Area: Clone,
{
    let result = perform_task(t, Rc::new(List::Empty))?;
    println!(w, "{}", result).unwrap();
    Ok(())
}

