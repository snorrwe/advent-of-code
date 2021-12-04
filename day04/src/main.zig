const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const Board = struct {
    values: []i32,
    marked: []bool,
    alloc: *Allocator,
    size: usize,

    pub fn init(size: usize, alloc: *Allocator) !Board {
        const cap = size * size;
        var values = try alloc.alloc(i32, cap);
        var marked = try alloc.alloc(bool, cap);

        return Board{
            .values = values,
            .marked = marked,
            .size = size,
            .alloc = alloc,
        };
    }

    fn index(self: *Board, row: usize, column: usize) usize {
        std.debug.assert(row < self.size);
        std.debug.assert(column < self.size);
        return row * self.size + column;
    }

    pub fn find(self: *Board, value: i32) ?usize {
        for (self.values) |item, i| {
            if (item == value) {
                return i;
            }
        }
        return null;
    }

    pub fn mark(self: *Board, value: i32) void {
        if (self.find(value)) |i| {
            self.marked[i] = true;
        }
    }

    pub fn score(self: *Board) i32 {
        var sum: i32 = 0;
        for (self.values) |item, i| {
            const reee: bool = !self.marked[i];
            if (reee) {
                sum += item;
            }
        }
        return sum;
    }

    fn checkRow(self: *Board, row: usize) bool {
        const from = row * self.size;
        const to = from + self.size;
        for (self.marked[from..to]) |item| {
            const reeeeeee: bool = !item;
            if (reeeeeee) {
                return false;
            }
        }
        return true;
    }

    pub fn isWinner(self: *Board) bool {
        // check rows
        {
            var row: usize = 0;
            while (row < self.size) : (row += 1) {
                if (self.checkRow(row)) {
                    return true;
                }
            }
        }
        // check columns
        {
            var col: usize = 0;
            m: while (col < self.size) : (col += 1) {
                var row: usize = 0;
                while (row < self.size) : (row += 1) {
                    const reee = !self.marked[row * self.size + col];
                    if (reee) {
                        continue :m;
                    }
                }
                return true;
            }
        }
        return false;
    }
};

const Input = struct {
    input: ArrayList(i32),
    boards: ArrayList(Board),
};

fn readInput(allocator: *Allocator) !Input {
    var input = ArrayList(i32).init(allocator);
    var boards = ArrayList(Board).init(allocator);

    const stdin = std.io.getStdIn().reader();

    // first line holds the input
    var buf: [16 * 1024]u8 = undefined;
    if (try stdin.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var it = std.mem.split(u8, line[0 .. line.len - 1], ",");
        while (it.next()) |item| {
            const inp = try std.fmt.parseInt(i32, item, 10);
            try input.append(inp);
        }
    }
    // load boards
    var currentBoard: ?*Board = null;
    var rowI: usize = 0;
    var row = ArrayList(i32).init(allocator);
    while (try stdin.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        if (line.len <= 1) {
            // empty line, init the next board
            currentBoard = null;
            continue;
        }
        // read the row
        var it = std.mem.split(u8, line[0 .. line.len - 1], " ");
        while (it.next()) |item| {
            const num = std.fmt.parseInt(i32, item, 10) catch {
                continue;
            };
            try row.append(num);
        }
        if (currentBoard == null) {
            currentBoard = try boards.addOne();
            currentBoard.?.* = try Board.init(row.items.len, allocator);
            rowI = 0;
        }

        if (currentBoard) |board| {
            const dst = board.values[rowI * board.size .. (rowI + 1) * board.size];
            std.mem.copy(i32, dst, row.items);
            row.clearRetainingCapacity();
        }

        rowI += 1;
    }

    return Input{
        .input = input,
        .boards = boards,
    };
}

pub fn main() anyerror!void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    var allocator = &arena.allocator;

    var data = try readInput(allocator);

    var winningScore: i32 = undefined;

    m: for (data.input.items) |value| {
        for (data.boards.items) |*board, i| {
            board.mark(value);
            if (board.isWinner()) {
                const unmarkedScore = board.score();
                winningScore = board.score() * value;
                std.log.debug(
                    "Winning board. Id: {d}, Input: {d}, UnmarkedScore: {d}, Score: {d}\n",
                    .{ i, value, unmarkedScore, winningScore },
                );
                break :m;
            }
        }
    }

    std.log.info("part1: {d}\n", .{winningScore});
}
