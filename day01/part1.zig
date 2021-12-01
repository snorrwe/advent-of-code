const std = @import("std");
const io = std.io;

pub fn main() anyerror!void {
    const file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var buf_reader = io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();
    var buf: [128]u8 = undefined;

    const first = (try in_stream.readUntilDelimiterOrEof(&buf, '\n')).?;
    var lastNum = try std.fmt.parseInt(i32, first, 10);

    var count: usize = 0;
    m: while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const newNum = std.fmt.parseInt(i32, line, 10) catch |err| {
            std.debug.print("sadge {s} {e}\n", .{line, err});
            break :m;
        };
        if (newNum > lastNum) {
            count += 1;
        }
        lastNum = newNum;
    }

    std.debug.print("poggers {d}\n", .{count});
}
