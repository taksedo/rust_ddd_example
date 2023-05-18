use common_types::main::base::domain_entity::Version;
use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::MealName;

/// На данный момент эта dto используется в нескольких сценариях.
/// Тут следует быть осторожным и вовремя заметить, когда разным сценариям нужен будет разный набор данных
/// Если такое произойдёт — необходимо выделить отдельный класс. В нашем случае можно оставить и так.
#[derive(Debug, PartialEq)]
pub struct MealInfo {
    pub id: MealId,
    pub name: MealName,
    // pub description: MealDescription,
    // pub price: Price,
    pub version: Version,
}
