const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    // Standard target options allows the person running `zig build` to choose
    // what target to build for. Here we do not override the defaults, which
    // means any target is allowed, and the default is native. Other options
    // for restricting supported target set are available.
    const target = b.standardTargetOptions(.{});

    // Standard release options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall.
    const mode = b.standardReleaseOptions();

    const exe = b.addExecutable("day01p1", "part1/main.zig");
    exe.setTarget(target);
    exe.setBuildMode(mode);
    exe.install();

    const run_cmd1 = exe.run();
    run_cmd1.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd1.addArgs(args);
    }
    const run_p1 = b.step("run1", "Run part 1");
    run_p1.dependOn(&run_cmd1.step);

    const exe2 = b.addExecutable("day01p2", "part2/main.zig");
    exe2.setTarget(target);
    exe2.setBuildMode(mode);
    exe2.install();

    const run_cmd2 = exe2.run();
    run_cmd2.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd2.addArgs(args);
    }

    const run_p2 = b.step("run2", "Run part 2");
    run_p2.dependOn(&run_cmd2.step);
}
