#include "gtest/gtest.h"

int ft_strlen(char* str);

TEST(strlen, empty) {
    char buf[] = "";
    EXPECT_EQ(ft_strlen(buf), 0);
}

TEST(strlen, short) {
    char buf[] = "short string";
    EXPECT_EQ(ft_strlen(buf), 12);
}

TEST(strlen, long) {
    char buf[] = "long string long string long string long string long string long string "
        "long string long string long string long string long string long string "
        "long string long string long string long string long string long string "
        "long string long string long string long string long string long string "
        "long string long string long string long string long string long string "
        "long string long string long string long string long string long string "
        "long string long string long string long string long string long string "
        "long string long string long string long string long string long string "
        "long string long string long string long string long string long string "
        "long string long string long string long string long string long string ";
    EXPECT_EQ(ft_strlen(buf), 720);
}

TEST(strlen, middle_null) {
    char buf[] = "Weird\0string";
    EXPECT_EQ(ft_strlen(buf), 5);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
