mod constants;
mod sum_squares_perf_test;
mod introductions;

fn main() {
    let do_introductions = false;

    if do_introductions {
        introductions::do_introductions()
    }

    sum_squares_perf_test::compare_performance();
    sum_squares_perf_test::display_graph();
}
