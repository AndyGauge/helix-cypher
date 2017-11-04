class CypherController < ApplicationController
  def index
    @text = params[:text] || ""
  end

  def create
    @text = TextTransform.rot13(params[:text])
    render :index
  end
end
