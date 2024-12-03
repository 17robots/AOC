const std = @import("std");
const util = @import("util");

const range = 3;
const tolerance = 1;
pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();
    var safes: u32 = 0;
    safes = 1;
    var lines = std.mem.tokenizeAny(u8, try util.read_file(alloc, "src/day2/2.txt"), "\n\r");
    while (lines.next()) |line| {
        var prev: ?i32 = null;
        var currentDif: ?i32 = null;
        var numbers = std.mem.tokenizeAny(u8, line, " ");
        while (numbers.next()) |n| {
            const x = try std.fmt.parseInt(i32, n, 10);
            if (prev) |p| {
                currentDif = x - p;
            }
            prev = x;
        }
    }
    std.debug.print("safes: {}\n", .{safes});
}
