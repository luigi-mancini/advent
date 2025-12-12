const std = @import("std");
const ArrayList = std.ArrayList;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

pub fn parse(comptime T: type, str: []const u8, list: *ArrayList(ArrayList(T))) !void {
    const init = list.*.items.len == 0;

    var it = std.mem.tokenizeAny(u8, str, " ");

    var index: usize = 0;

    while (it.next()) |val| : (index += 1) {
        const operand = switch (type) {
            u64 => try std.fmt.parseInt(T, val, 10),
            u8 => val,
            else => val,
        };

        //

        if (init) {
            try list.append(allocator, try ArrayList(T).initCapacity(allocator, 10));
        }

        std.debug.assert(index < list.items.len);

        try list.items[index].append(allocator, operand);
    }
}

pub fn parseOperands(str: []u8, list: *ArrayList(ArrayList(u64))) !void {
    const init = list.*.items.len == 0;

    var it = std.mem.tokenizeAny(u8, str, " ");

    var index: usize = 0;

    while (it.next()) |val| : (index += 1) {
        const operand = try std.fmt.parseInt(u64, val, 10);

        if (init) {
            try list.append(allocator, try ArrayList(u64).initCapacity(allocator, 10));
        }

        std.debug.assert(index < list.items.len);

        list.items[index] = operand;
    }
}

pub fn readInput(operands: *ArrayList(ArrayList(u64)), operators: *ArrayList(ArrayList(u8))) !void {
    const cwd = std.fs.cwd();
    const file = try cwd.openFile("test.txt", .{ .mode = .read_only });
    defer file.close();

    var read_buffer: [1024]u8 = undefined;
    var fr = file.reader(&read_buffer);
    var reader = &fr.interface;

    while (reader.takeDelimiterExclusive('\n')) |line| {
        const str = std.mem.trimRight(u8, line, "\r\n");

        if (str.len != 0) {
            if (std.ascii.isDigit(str[0])) {
                try parse(u64, str, operands);
            } else {
                try parse(u8, str, operators);
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
    //defer _ = gpa.deinit();

    var operands = try ArrayList(ArrayList(u64)).initCapacity(allocator, 500);
    var operators = try ArrayList(ArrayList(u8)).initCapacity(allocator, 500);
    defer operands.deinit(allocator);
    defer operators.deinit(allocator);

    try readInput(&operands, &operators);

    for (operands.items) |op| {
        std.debug.print("LDB {}\n", .{op});
    }

    //for (ids.items) |i| {
    //    std.debug.print("{d}\n", .{i});
    //}

    // const count = countFreshIngredients(mergedRanges.items, ids.items);
    //std.debug.print("Part 1: We have {d} fresh ingredients.\n", .{count});

    //std.debug.print("Part 2: We have {d} fresh ingredients.\n", .{mergedCount});
}
