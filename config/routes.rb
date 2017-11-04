Rails.application.routes.draw do
  resources :cypher, path: '/', only: [:index, :create]
end
