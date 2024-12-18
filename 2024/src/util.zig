const std = @import("std");
pub fn read_file(a: std.mem.Allocator, filename: []const u8) ![]const u8 {
    const file = try std.fs.cwd().openFile(filename, .{ .mode = .read_only });
    defer file.close();
    const stat = try file.stat();
    return try file.readToEndAlloc(a, stat.size);
}
pub fn all(comptime t: type, list: []t, func: anytype) bool {
    for (list) |i| {
        if (!@call(.auto, func, .{i})) return false;
    }
    return true;
}
