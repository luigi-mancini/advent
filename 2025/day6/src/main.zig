const std = @import("std");
const ArrayList = std.ArrayList;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

const Range = struct { start: u64, end: u64 };

pub fn doRangesIntersect(r1: Range, r2: Range) bool {
    if ((r1.start >= r2.start and r1.start <= r2.end) or
        (r1.end >= r2.start and r1.end <= r2.end) or
        (r2.start >= r1.start and r2.start <= r1.end) or
        (r2.end >= r1.start and r2.end <= r1.end))
    {
        return true;
    }
    return false;
}

pub fn mergeRanges(r1: Range, r2: Range) Range {
    return .{ .start = @min(r1.start, r2.start), .end = @max(r1.end, r2.end) };
}

pub fn addAndMergeRangeToList(r: Range, list: *ArrayList(Range)) !void {
    var range: Range = r;

    var index: usize = 0;

    while (index < list.items.len) {
        if (doRangesIntersect(range, list.items[index])) {
            range = mergeRanges(range, list.items[index]);
            _ = list.swapRemove(index);
        } else {
            index += 1;
        }
    }
    try list.append(allocator, range);
}

pub fn countFreshIngredients(ranges: []Range, ids: []u64) u64 {
    var count: u64 = 0;
    for (ids) |i| {
        for (ranges) |r| {
            if (i >= r.start and i <= r.end) {
                count += 1;
                break;
            }
        }
    }
    return count;
}

pub fn parseOperands(
    str: []u8,
) !void {
    var it = std.mem.tokenizeAny(u8, str, " ");

    while (it.next()) |val| {
        const operand = try std.fmt.parseInt(u64, val, 10);
        try list.append(allocator, operand);
    }
}

pub fn readInput(operands: *ArrayList(ArrayList(u64)), operators: *ArrayList(ArrayList(u8))) !void {
    const cwd = std.fs.cwd();
    const file = try cwd.openFile("day5.txt", .{ .mode = .read_only });
    defer file.close();

    var read_buffer: [1024]u8 = undefined;
    var fr = file.reader(&read_buffer);
    var reader = &fr.interface;

    while (reader.takeDelimiterExclusive('\n')) |line| {
        const str = std.mem.trimRight(u8, line, "\r\n");

        if (str.len != 0) {
            if (std.ascii.isDigit(str[0])) {
                parseOperands();
            } else {
                parseOperators();
            }
        }

        if (std.mem.indexOfScalar(u8, str, '-')) |index| {
            const start = try std.fmt.parseInt(u64, str[0..index], 10);
            const end = try std.fmt.parseInt(u64, str[index + 1 ..], 10);

            const r: Range = .{ .start = start, .end = end };
            try addAndMergeRangeToList(r, mergedRange);
        } else {
            if (str.len != 0) {
                const id = try std.fmt.parseInt(u64, str, 10);
                try ids.append(allocator, id);
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
    var operators = try ArrayList(ArrayList(u8)).initCapacity(allocator, 500);
    defer operands.deinit(allocator);
    defer operators.deinit(allocator);

    try readInput(&operands, &operators);

    //for (ranges.items) |r| {
    //    std.debug.print("{}\n", .{r});
    //}

    //for (ids.items) |i| {
    //    std.debug.print("{d}\n", .{i});
    //}

    const count = countFreshIngredients(mergedRanges.items, ids.items);
    std.debug.print("Part 1: We have {d} fresh ingredients.\n", .{count});

    var mergedCount: u64 = 0;
    for (mergedRanges.items) |r| {
        mergedCount += (r.end - r.start) + 1;
    }

    std.debug.print("Part 2: We have {d} fresh ingredients.\n", .{mergedCount});
}
