mod functions;
mod slack;
mod table;

fn main() {
    azure_functions::worker_main(std::env::args(), functions::EXPORTS);
}
