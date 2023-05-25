use application::main::event::event_publisher_impl::EventPublisherImpl;
use domain::main::menu::meal_events::DomainEventEnum;
use domain::main::menu::meal_id::MealIdGenerator;
use in_memory_persistence::main::menu::in_memory_meal_repository::InMemoryMealRepository;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use usecase::main::menu::access::meal_extractor::MealExtractor;
use usecase::main::menu::access::meal_persister::MealPersister;
use usecase::main::menu::invariant::meal_already_exists_uses_meal_extractor::MealAlreadyExistsUsesMealExtractor;
use usecase::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;
use usecase::main::menu::scenario::get_meal_by_id_use_case::GetMealByIdUseCase;

pub fn meal_create_repository() -> Arc<Mutex<InMemoryMealRepository>> {
    let meal_publisher = EventPublisherImpl::<DomainEventEnum>::default();
    let meal_repository = Arc::new(Mutex::new(InMemoryMealRepository::new(Arc::new(
        Mutex::new(meal_publisher),
    ))));
    meal_repository
}

pub fn meal_create_shared_state<U, V>(
    meal_repository: Arc<Mutex<U>>,
    meal_id_generator: Arc<Mutex<V>>,
) -> Arc<Mutex<AddMealToMenuUseCase>>
where
    U: Debug + Send + MealExtractor + MealPersister + 'static,
    V: Debug + Send + MealIdGenerator + 'static,
{
    let rule = MealAlreadyExistsUsesMealExtractor::new(Arc::clone(&meal_repository) as _);

    let usecase = AddMealToMenuUseCase::new(
        Arc::clone(&meal_repository) as _,
        meal_id_generator,
        Arc::new(Mutex::new(rule)),
    );
    Arc::new(Mutex::new(usecase))
}

pub fn meal_get_by_id_shared_state<U>(
    meal_repository: Arc<Mutex<U>>,
) -> Arc<Mutex<GetMealByIdUseCase>>
where
    U: Debug + Send + MealExtractor + MealPersister + 'static,
{
    let usecase = GetMealByIdUseCase::new(Arc::clone(&meal_repository) as _);
    Arc::new(Mutex::new(usecase))
}
