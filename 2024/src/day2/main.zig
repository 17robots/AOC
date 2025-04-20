const std = @import("std");
const util = @import("util");

const range = 3;
const tolerance = 1;
pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();
    var safes: u32 = 0;
    var lines = std.mem.tokenizeAny(u8, try util.read_file(alloc, "src/day2/2.txt"), "\n\r");
    while (lines.next()) |line| {
        var numbers = std.mem.tokenizeAny(u8, line, " ");
        var nums = std.ArrayList(i32).init(alloc);
        while (numbers.next()) |n| try nums.append(try std.fmt.parseInt(i32, n, 10));
        if (try safe(alloc, nums.items)) {
            safes += 1;
        } else {
            for (0..nums.items.len) |i| {
                var num_clone = try nums.clone();
                var candidate = std.ArrayList(i32).fromOwnedSlice(alloc, num_clone.items[0..i]);
                try candidate.appendSlice(num_clone.items[i + 1 ..]);
                if (try safe(alloc, candidate.items)) {
                    safes += 1;
                    break;
                } else {}
            }
        }
    }
    std.debug.print("safes: {}\n", .{safes});
}

pub fn safe(alloc: std.mem.Allocator, nums: []i32) !bool {
    var prev: ?i32 = null;
    var diffs = std.ArrayList(i32).init(alloc);
    defer diffs.deinit();
    for (nums) |n| {
        if (prev) |p| try diffs.append(n - p);
        prev = n;
    }
    return util.all(i32, diffs.items, struct {
        fn func(i: i32) bool {
            return i > 0 and i <= 3;
        }
    }.func) or util.all(i32, diffs.items, struct {
        fn func(i: i32) bool {
            return i < 0 and i >= -3;
        }
    }.func);
}
