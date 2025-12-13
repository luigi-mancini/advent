const std = @import("std");
const ArrayList = std.ArrayList;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

pub fn parseOperators(str: []const u8, list: *ArrayList(u8)) !void {
    var it = std.mem.tokenizeAny(u8, str, " ");

    while (it.next()) |val| {
        std.debug.assert(val.len == 1);

        try list.append(allocator, val[0]);
    }
}

pub fn parseOperands(str: []const u8, list: *ArrayList(ArrayList(u64))) !void {
    const init = list.*.items.len == 0;
    var it = std.mem.tokenizeAny(u8, str, " ");

    var index: usize = 0;
    while (it.next()) |val| : (index += 1) {
        const operand = try std.fmt.parseInt(u64, val, 10);

        if (init) {
            try list.append(allocator, try ArrayList(u64).initCapacity(allocator, 10));
        }

        std.debug.assert(index < list.items.len);
        try list.items[index].append(allocator, operand);
    }
}

pub fn createOperandsFromTable(table: [][]u8, list: *ArrayList(ArrayList(u64))) !void {
    var index: usize = 0;
    var str = try std.ArrayList(u8).initCapacity(allocator, 10);
    defer str.deinit(allocator);

    for (0..table[0].len) |x| {
        for (0..table.len) |y| {
            if (std.ascii.isDigit(table[y][x])) {
                try str.append(allocator, table[y][x]);
            }
        }

        if (str.items.len == 0) {
            index += 1;
        } else {
            if (index == list.items.len) {
                try list.append(allocator, try ArrayList(u64).initCapacity(allocator, 10));
            }
            const val = try std.fmt.parseInt(u64, str.items, 10);
            try list.items[index].append(allocator, val);
        }

        str.clearRetainingCapacity();
    }
}

pub fn addList(list: []u64) u64 {
    var ret: u64 = 0;
    for (list) |val| {
        ret += val;
    }

    return ret;
}

pub fn multList(list: []u64) u64 {
    var ret: u64 = 1;
    for (list) |val| {
        ret *= val;
    }

    return ret;
}

pub fn computeAnswer(operands: []ArrayList(u64), operators: []u8) u64 {
    std.debug.assert(operands.len == operators.len);

    var answer: u64 = 0;
    for (0..operands.len) |index| {
        answer += switch (operators[index]) {
            '+' => addList(operands[index].items),
            '*' => multList(operands[index].items),
            else => {
                @panic("Unexpected operator");
            },
        };
    }

    return answer;
}

pub fn readInput(operands: *ArrayList(ArrayList(u64)), operators: *ArrayList(u8), table: *ArrayList([]u8)) !void {
    const cwd = std.fs.cwd();
    const file = try cwd.openFile("day6.txt", .{ .mode = .read_only });
    defer file.close();

    var read_buffer: [1024 * 10]u8 = undefined;
    var fr = file.reader(&read_buffer);
    var reader = &fr.interface;

    while (reader.takeDelimiterExclusive('\n')) |line| {
        const str = std.mem.trim(u8, line, "\r\n");
        const trimmed = std.mem.trim(u8, line, " ");

        if (str.len != 0) {
            if (str[0] == '*' or str[0] == '+') {
                try parseOperators(trimmed, operators);
            } else {
                const tmp = try allocator.dupe(u8, str);
                try table.append(allocator, tmp);
                try parseOperands(trimmed, operands);
            }
        }
        _ = try reader.discardDelimiterInclusive('\n');
    } else |err| switch (err) {
        error.EndOfStream => {},
        error.StreamTooLong, // line could not fit in buffer
        error.ReadFailed, // caller can check reader implementation for diagnostics
        => |e| return e,
    }
}

pub fn main() !void {
    defer _ = gpa.deinit();

    var operands = try ArrayList(ArrayList(u64)).initCapacity(allocator, 500);
    var operators = try ArrayList(u8).initCapacity(allocator, 500);
    var table = try ArrayList([]u8).initCapacity(allocator, 500);
    var operands_part2 = try ArrayList(ArrayList(u64)).initCapacity(allocator, 500);

    defer {
        for (operands.items) |*tmp| {
            tmp.deinit(allocator);
        }
        operands.deinit(allocator);
        operators.deinit(allocator);

        for (table.items) |tmp| {
            allocator.free(tmp);
        }
        table.deinit(allocator);

        for (operands_part2.items) |*tmp| {
            tmp.deinit(allocator);
        }
        operands_part2.deinit(allocator);
    }

    try readInput(&operands, &operators, &table);

    // for (table.items) |op| {
    //    std.debug.print("{s}\n", .{op});
    // }

    const answer: u64 = computeAnswer(operands.items, operators.items);
    std.debug.print("Part 1: The answer is {d}.\n", .{answer});

    try createOperandsFromTable(table.items, &operands_part2);
    const answer_part2: u64 = computeAnswer(operands_part2.items, operators.items);
    std.debug.print("Part 2: The answer is {d}.\n", .{answer_part2});
}
