/***

// See https://github.com/rust-lang/rust-dbg-ext/blob/main/test-framework/dbt/README.md
// for more information on how these test scripts work.

#if @cdb
  // Start running the program
  g

  // The following line checks that we actually hit the breakpoint.
  #check Breakpoint 0 hit

  // Check that we get a nice list of tasks, and the name of the corresponding async fns
  dx person
  #check pool.pool.task_stacks : [num_tasks = 3]
  dx person2
  #check pool.pool.task_stacks : [num_tasks = 3]

  q

// Only run this test on Windows
#if not @cdb
  #ignore-test

***/

use my_lib::Person;


fn main() {
    let person = Person::new("John Doe", 10);
    let person2 = Person::new("Jane Doe", 1);

    zzz(); // #break
}

#[inline(never)]
pub fn zzz() {}
