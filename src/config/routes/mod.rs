
macro_rules! resource_routes {
    ($router:ident, $model:ident) => {
        {
            let class_name = stringify!($model).replace("_controller", "").to_string().to_kebab_case().to_plural();
            $router.route(format!("/{}", class_name).as_str(), get($model::index))
            .route(format!("/{}/new", class_name).as_str(), get($model::new))
            .route(format!("/{}/:id", class_name).as_str(), get($model::show))
            .route(format!("/{}/:id", class_name).as_str(), delete($model::delete))
            .route(format!("/{}/:id/edit", class_name).as_str(), get($model::edit))
            .route(format!("/{}", class_name).as_str(), post($model::create))
            .route(format!("/{}/:id", class_name).as_str(), post($model::update))
        }  
    };
}

pub mod web;
