#include <CLI/Validators.hpp>
#include <iostream>
#include <csignal>
#include <CLI/CLI.hpp>

using std::cout;
using std::cerr;
using std::string;

extern "C" {
    #include <pluto.h>
}

void clean_exit(int s) {
    cerr << "recived signal " << s << "! exiting cleanly...\n";
    pluto_deinit();
    exit(-1);
}

int main(int argc, char** argv) {
    // ensure clean exit
    signal(SIGINT, clean_exit);
    signal(SIGTERM, clean_exit);
    
    // parse arguments
    bool exit_immediatly = false;
    bool loop = false;
    string file;

    CLI::App app{"Terminal image_viewer"};
    app.add_flag("--exit-immediatly,-e", exit_immediatly, "Exits immediatly instead of waiting for keyboard input.");
    app.add_flag("--disable-loop,-l", loop, "Disables looping animated images (like GIFs).");
    app.add_option("image", file, "The image file to display.")
        ->check(CLI::ExistingFile)
        ->required(true);
    CLI11_PARSE(app, argc, argv);

    // TODO parse file


    // TODO display file
}