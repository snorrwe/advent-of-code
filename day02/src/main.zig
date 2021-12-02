const std = @import("std");
const io = std.io;
const Vector = std.meta.Vector;

const IVec2 = Vector(2, i32);
const IVec3 = Vector(3, i32);

pub fn main() anyerror!void {
    const file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    const p1 = try part1(file);
    std.debug.print("p1: {d}\n", .{p1[0] * p1[1]});
}

fn part1(file: std.fs.File) anyerror!IVec2 {
    var buf_reader = io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();
    var buf: [128]u8 = undefined;

    var pos: IVec2 = .{ 0, 0 };
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var it = std.mem.split(line, " ");

        const ty = it.next().?;
        const valStr = it.next().?;

        const val = try std.fmt.parseInt(i32, valStr[0 .. valStr.len - 1], 10);

        if (std.mem.eql(u8, ty, "forward")) {
            pos[0] += val;
        } else if (std.mem.eql(u8, ty, "up")) {
            pos[1] -= val;
        } else if (std.mem.eql(u8, ty, "down")) {
            pos[1] += val;
        }
    }

    return pos;
}
