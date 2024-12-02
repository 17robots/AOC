const std = @import("std");
const util = @import("util");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();
    var lines = std.mem.tokenizeAny(u8, try util.read_file(alloc, "src/day1/1.txt"), " \n\r");
    var read_list1 = std.ArrayList(usize).init(alloc);
    defer read_list1.deinit();
    var read_list2 = std.ArrayList(usize).init(alloc);
    defer read_list2.deinit();
    var sum: u64 = 0;
    var i: u32 = 0;
    while (lines.next()) |line| : (i += 1) {
        const x = std.fmt.parseInt(u32, line, 10) catch blk: {
            std.debug.print("Failed to parse: '{s}'\n", .{line});
            break :blk 0;
        };
        if (@mod(i, 2) == 0) {
            try read_list1.append(x);
        } else {
            try read_list2.append(x);
        }
    }
    std.mem.sort(usize, read_list1.items, {}, comptime std.sort.asc(usize));
    std.mem.sort(usize, read_list2.items, {}, comptime std.sort.asc(usize));
    for (read_list1.items) |j| sum += j * count_in_arr(usize, read_list2.items, j);
    std.debug.print("sum: {}\n", .{sum});
}

pub fn count_in_arr(comptime t: type, collection: []t, val: t) usize {
    var count: usize = 0;
    for (collection) |i| if (val == i) {
        count += 1;
    } else {};
    return count;
}
