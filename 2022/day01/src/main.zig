const std = @import("std");

pub fn part1(input: []const u8) anyerror!i32 {
    var max_calories: i32 = 0;
    var current_calories: i32 = 0;
    var it = std.mem.split(u8, input, "\n");
    while (it.next()) |line| {
        if (line.len == 0) {
            if (max_calories < current_calories) {
                max_calories = current_calories;
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

    var p1 = try part1(input);

    std.log.info("part1: {}", .{p1});
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

    var result = try part1(input);
    var expected: i32 = 24000;
    try std.testing.expectEqual(expected, result);
}
