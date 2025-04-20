const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const util_import = b.addModule("util", .{ .target = target, .optimize = optimize, .root_source_file = b.path("src/util.zig") });

    for (1..4) |i| {
        const day = b.addExecutable(.{ .name = b.fmt("day{d}", .{i}), .root_source_file = b.path(b.fmt("src/day{d}/main.zig", .{i})), .target = target, .optimize = optimize });
        day.root_module.addImport("util", util_import);
        const install_day = b.addRunArtifact(day);
        const day_step = b.step(b.fmt("day{d}", .{i}), b.fmt("Run day {d}", .{i}));
        day_step.dependOn(&day.step);
        day_step.dependOn(&install_day.step);
    }
}
