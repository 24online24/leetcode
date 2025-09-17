use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct FoodRatings {
    foods_info: HashMap<String, (String, i32)>,
    cuisines_foods_ratings: HashMap<String, BinaryHeap<(i32, Reverse<String>)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut foods_info = HashMap::new();
        let mut cuisines_foods_ratings = HashMap::new();
        for (cuisine, (food, rating)) in cuisines
            .into_iter()
            .zip(foods.into_iter().zip(ratings.into_iter()))
        {
            foods_info.insert(food.clone(), (cuisine.clone(), rating));
            cuisines_foods_ratings
                .entry(cuisine)
                .or_insert_with(BinaryHeap::new)
                .push((rating, Reverse(food)));
        }
        FoodRatings {
            foods_info,
            cuisines_foods_ratings,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        if let Some(food_info) = self.foods_info.get_mut(&food) {
            food_info.1 = new_rating;
            let cuisine_ratings = self.cuisines_foods_ratings.get_mut(&food_info.0).unwrap();
            cuisine_ratings.push((new_rating, Reverse(food)));
        }
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        if let Some(cuisine_ratings) = self.cuisines_foods_ratings.get_mut(&cuisine) {
            while let Some(food) = cuisine_ratings.peek() {
                if food.0 == self.foods_info.get(&food.1.0).unwrap().1 {
                    return food.1.0.clone();
                } else {
                    cuisine_ratings.pop();
                }
            }
        }
        "".to_owned()
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut food_ratings = FoodRatings::new(
            vec![
                "kimchi".to_string(),
                "miso".to_string(),
                "sushi".to_string(),
                "moussaka".to_string(),
                "ramen".to_string(),
                "bulgogi".to_string(),
            ],
            vec![
                "korean".to_string(),
                "japanese".to_string(),
                "japanese".to_string(),
                "greek".to_string(),
                "japanese".to_string(),
                "korean".to_string(),
            ],
            vec![9, 12, 8, 15, 14, 7],
        );
        assert_eq!("kimchi", food_ratings.highest_rated("korean".to_string()));
        assert_eq!("ramen", food_ratings.highest_rated("japanese".to_string()));
        food_ratings.change_rating("sushi".to_owned(), 16);
        assert_eq!("sushi", food_ratings.highest_rated("japanese".to_string()));
        food_ratings.change_rating("ramen".to_owned(), 16);
        assert_eq!("ramen", food_ratings.highest_rated("japanese".to_string()));
    }

    #[test]
    fn example_37() {
        let mut food_ratings = FoodRatings::new(
            vec![
                "emgqdbo".to_string(),
                "jmvfxjohq".to_string(),
                "qnvseohnoe".to_string(),
                "yhptazyko".to_string(),
                "ocqmvmwjq".to_string(),
            ],
            vec![
                "snaxol".to_string(),
                "snaxol".to_string(),
                "snaxol".to_string(),
                "fajbervsj".to_string(),
                "fajbervsj".to_string(),
            ],
            vec![2, 6, 18, 6, 5],
        );
        food_ratings.change_rating("qnvseohnoe".to_owned(), 11);
        assert_eq!(
            "yhptazyko",
            food_ratings.highest_rated("fajbervsj".to_string())
        );
        food_ratings.change_rating("emgqdbo".to_owned(), 3);
        food_ratings.change_rating("jmvfxjohq".to_owned(), 9);
        food_ratings.change_rating("emgqdbo".to_owned(), 14);
        assert_eq!(
            "yhptazyko",
            food_ratings.highest_rated("fajbervsj".to_string())
        );
        assert_eq!("emgqdbo", food_ratings.highest_rated("snaxol".to_string()));
    }

    #[test]
    fn example_37_simplified() {
        let mut food_ratings = FoodRatings::new(
            vec![
                "food_1".to_string(),
                "food_2".to_string(),
                "food_3".to_string(),
            ],
            vec![
                "only_cuisine".to_string(),
                "only_cuisine".to_string(),
                "only_cuisine".to_string(),
            ],
            vec![2, 6, 18],
        );
        food_ratings.change_rating("food_3".to_owned(), 11);
        food_ratings.change_rating("food_1".to_owned(), 3);
        food_ratings.change_rating("food_2".to_owned(), 9);
        food_ratings.change_rating("food_1".to_owned(), 14);
        assert_eq!(
            "food_1",
            food_ratings.highest_rated("only_cuisine".to_string())
        );
    }
}
