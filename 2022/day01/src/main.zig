const std = @import("std");

pub fn solve(input: []const u8) anyerror![3]i32 {
    var max_calories: [3]i32 = [_]i32{ 0, 0, 0 };
    var current_calories: i32 = 0;
    var it = std.mem.split(u8, input, "\n");
    while (it.next()) |line| {
        if (line.len == 0) {
            for (max_calories) |*v, i| {
                if (v.* < current_calories) {
                    std.mem.rotate(i32, max_calories[0 .. i + 1], i);
                    v.* = current_calories;
                }
            }
            current_calories = 0;
        } else {
            current_calories += try std.fmt.parseInt(i32, line, 10);
        }
    }
    return max_calories;
}

pub fn main() anyerror!void {
    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    var a = gpa.allocator();

    var input = try file.readToEndAlloc(a, 32000);

    var result = try solve(input);

    std.log.info("part1: {}", .{result[2]});
    var sum: i32 = 0;
    for (result) |n| {
        sum += n;
    }
    std.log.info("part2: {}", .{sum});
}

test "part1" {
    const input =
        \\1000
        \\2000
        \\3000
        \\
        \\4000
        \\
        \\5000
        \\6000
        \\
        \\7000
        \\8000
        \\9000
        \\
        \\10000
    ;

    var result = try solve(input);
    var expected: i32 = 24000;
    try std.testing.expectEqual(expected, result[2]);
}

test "part2" {
    const input =
        \\1000
        \\2000
        \\3000
        \\
        \\4000
        \\
        \\5000
        \\6000
        \\
        \\7000
        \\8000
        \\9000
        \\
        \\10000
    ;

    var result = try solve(input);
    var sum: i32 = 0;
    for (result) |n| {
        sum += n;
    }
    var expected: i32 = 45000;
    try std.testing.expectEqual(expected, sum);
}
