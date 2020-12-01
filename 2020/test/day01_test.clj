(ns day01-test
  (:require [clojure.test :refer [is deftest testing]]
            [day01 :as sut]))

(def example
  "1721
979
366
299
675
1456")

(def my-input (slurp "resources/input01.txt"))

(deftest part1
  (testing "example"
    (is (= 514579 (sut/part1 example))))
  (testing "my input"
    (is (= 1010299 (sut/part1 my-input)))))

(deftest part2
  (testing "example"
    (is (= 241861950 (sut/part2 example))))
  (testing "my input"
    (is (= 42140160 (sut/part2 my-input)))))

(comment
  (clojure.test/run-tests *ns*))
