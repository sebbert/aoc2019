(use '[clojure.pprint :only (pprint)])

(defn parse-input [input]
  (->> input
    (clojure.string/split-lines)
    (map (fn [str] (clojure.string/split str #"\)")))))

(defn map-values [f coll & args]
  (reduce-kv (fn [acc k v] (assoc acc k (apply f v args))) {} coll))

(def input (slurp "./input"))

(def orbit-pairs (parse-input input))

(def parent-to-children
  (->> orbit-pairs
    (group-by first)
    (map-values (partial map second))))
(def child-to-parent
  (->> orbit-pairs
    (map (fn [[p c]] [c p]))
    (into {})))

(defn count-orbits
  ([parent] (count-orbits parent 0))
  ([parent depth]
    (let [children (or (parent-to-children parent) ())]
      (+ depth
        (reduce
          (fn [acc next] (+ acc (count-orbits next (inc depth))))
          0
          children)))))

(println "Part 1:" (count-orbits "COM"))

(defn search-forward [start end depth]
  (some
    (fn [next]
      (if (= next end)
        depth
        (search-forward next end (inc depth))))
    (parent-to-children start)))

(defn shortest-path-between [start end]
  (loop [backtrack-depth 0
         cur start]
    (when-let [parent (child-to-parent cur)]
      (or (search-forward parent end backtrack-depth)
          (recur (inc backtrack-depth) parent)))))

(println "Part 2:" (shortest-path-between "SAN" "YOU"))