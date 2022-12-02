package de.hamburgchimps.krampus.twentytwentytwo;

import java.util.List;
import java.util.function.Supplier;

public final class Solution {

    private static final List<List<Supplier<Result>>> store = List.of(DayOne.store);

    public static Result execute(int day, int part) {
        return store
                .get(day - 1)
                .get(part - 1)
                .get();
    }
    public static final class DayOne {
        // TODO: @next read input data`
        private static final List<Supplier<Result>> store = List.of(DayOne::PartOne, DayOne::PartTwo);

        private static Result PartOne() {
            return new Result(true);
        }

        private static Result PartTwo() {
            return new Result(true);
        }
    }

    public record Result(boolean success) {}
}
