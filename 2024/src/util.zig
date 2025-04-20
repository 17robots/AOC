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
pub fn any(comptime t: type, list: []t, func: anytype) bool {
    for (list) |i| {
        if (@call(.auto, func, .{i})) return true;
    }
    return false;
}
pub fn filter(alloc: std.mem.Allocator, comptime t: type, list: []t, func: anytype) ![]t {
    var x = std.ArrayList(t).init(alloc);
    for (list) |i| {
        if (@call(.auto, func, .{i})) try x.append(i);
    }
    return x.toOwnedSlice();
}
pub fn index_of(comptime t: type, items: []t, val: t, cmp: ?fn (t, t) bool) ?usize {
    for (0..items.len) |i| {
        if (cmp) |c| {
            if (@call(.auto, c, .{ items[i], val })) return i;
        } else {
            if (items[i] == val) return i;
        }
    }
    return null;
}

pub fn zip(alloc: std.mem.Allocator, comptime t: type, comptime t1: type, items1: []t, items2: []t1) std.AutoHashMap(t, t1) {
    var map = std.AutoHashMap(t, t1).init(alloc);
    for (0..if (items1.len < items2.len) items1.len else items2.len) |i| {
        try map.put(items1[i], items2[i]);
    }
    return map;
}
