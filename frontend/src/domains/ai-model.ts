export interface OpenAIModel {
  id: string;
  name: string;
  inputPricePer1m: number;
  outputPricePer1m: number;
  contextWindow: number;
  maxOutputTokens: number;
  isRecommended: boolean;
  recommendationReason?: string;
}

export interface ModelsListResponse {
  models: OpenAIModel[];
}
