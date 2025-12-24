{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # https://devenv.sh/basics/

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    just
    cookiecutter
    hyperfine
    lldb
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;
  languages.python = {
    enable = true;
    uv.enable = true;
    venv.enable = true;
    package = pkgs.python312;
  };
  languages.cplusplus.enable = true;
  env = {
    LD_LIBRARY_PATH = "${
      with pkgs;
      lib.makeLibraryPath [
        stdenv.cc.cc.lib
        libz
      ]
    }:$LD_LIBRARY_PATH";

  };

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
