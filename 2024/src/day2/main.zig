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
        var asc: ?bool = null;
        var prev: ?i32 = null;
        var unsafes: u32 = 0;
        while (numbers.next()) |n| blk: {
            const x = try std.fmt.parseInt(i32, n, 10);
            var skip_x = false;
            if (prev) |p| {
                if (asc) |a| {
                    switch (a) {
                        true => if (p > x) {
                            if (unsafes > tolerance) break :blk;
                            unsafes += 1;
                            skip_x = true;
                        },
                        false => if (p < x) {
                            if (unsafes > tolerance) break :blk;
                            unsafes += 1;
                            skip_x = true;
                        },
                    }
                } else {
                    asc = p < x;
                }
                switch (@abs(x - p)) {
                    1...range => {},
                    else => {
                        if (unsafes > tolerance) break :blk;
                        unsafes += 1;
                        skip_x = true;
                    },
                }
            }
            if (!skip_x) prev = x;
        }
        safes += if (unsafes > 1) 0 else 1;
    }
    std.debug.print("safes: {}", .{safes});
}
