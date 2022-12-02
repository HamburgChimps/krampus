package de.hamburgchimps.krampus.twentytwentytwo;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
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
        private static List<Integer> input = getInput("day1.txt");
        private static final List<Supplier<Result>> store = List.of(DayOne::PartOne, DayOne::PartTwo);

        private static Result PartOne() {
            return new Result(input, true);
        }

        private static Result PartTwo() {
            return new Result(input, true);
        }
    }

    public record Result(List<Integer> input, boolean success) {}

    private static List<Integer> getInput(String path) {
        try (var lines = Files.lines(Path.of(Solution.class.getClassLoader().getResource("input" + "/day1.txt").getPath()))) {
            return lines
                    .map(Integer::parseInt)
                    .toList();
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }
}
