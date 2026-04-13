#include "helpers.hpp"
#include "vyn_compilation.hpp"
#include <CLI/CLI.hpp>
#include <cstdlib>
#include <iostream>
#include <string>

struct Args {
  std::string input_file;
  std::string out_file;
  bool c_file;
};

Args get_args(int argc, char *argv[]) {
  CLI::App app{"Vyn compiler"};

  Args args;

  args.out_file = "a.out";
  args.c_file = false;

  app.add_option("input_file", args.input_file, "Input file")->required();

  auto out_opt = app.add_option("-o,--out", args.out_file, "Output file");

  app.add_flag("-c,--cfile", args.c_file,
               "Creates a c file instead of a binary");

  try {
    app.parse(argc, argv);
  } catch (const CLI::ParseError &err) {
    std::exit(app.exit(err));
  }

  if (args.c_file && out_opt->count() == 0) {
    args.out_file = "a.c";
  }

  return args;
}

int main(int argc, char *argv[]) {
  Args args = get_args(argc, argv);

  std::string contents = get_file_content(args.input_file);
  if (contents.empty()) {
    std::cerr << "Input file is empty or failed to read\n";
    return EXIT_FAILURE;
  }

  std::string c_code = vyn_to_c(contents);

  if (args.c_file) {
    std::string formatted_c_code = format_c(c_code);
    write_to_file(args.out_file, formatted_c_code);
  } else {
    compile_c(c_code, args.out_file);
  }

  return EXIT_SUCCESS;
}
