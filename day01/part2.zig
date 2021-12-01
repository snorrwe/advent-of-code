const std = @import("std");
const io = std.io;
const ArrayList = std.ArrayList;

pub fn main() anyerror!void {
    var gpalloc = std.heap.GeneralPurposeAllocator(.{}){};

    const file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var buf_reader = io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();
    var buf: [128]u8 = undefined;

    var numbers = ArrayList(i32).init(&gpalloc.allocator);
    defer numbers.deinit();

    m: while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const newNum = std.fmt.parseInt(i32, line, 10) catch |err| {
            std.debug.print("sadge {s} {e}\n", .{ line, err });
            break :m;
        };
        try numbers.append(newNum);
    }

    var count: usize = 0;

    var i: usize = 0;
    while (i < numbers.items.len - 3) {
        const s0 = sumWindow(numbers.items[i..]);
        const s1 = sumWindow(numbers.items[i+1..]);
        if (s0 < s1) {
            count += 1;
        }
        i += 1;
    }

    std.debug.print("poggers {d}\n", .{count});
}

fn sumWindow(items: []const i32) i32 {
    return items[0] + items[1] + items[2];
}
