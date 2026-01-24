import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useForm, Controller, useFieldArray } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { toast } from 'sonner';
import { categoryFormSchema, type CategoryFormData } from '@/schemas/category.schema';
import MultiChipInput, {
  getRandomColor,
} from '../components/inputs/multi-chip-input';
import type { UpdateCategoryModel } from '@/models/UpdateCategoryModel';
import type { CreateCategoryModel } from '@/models/CreateCategoryModel';
import { CategoryTypeEnum, type CategoryModel } from '@/domains/category';
import type { TagModel } from '@/domains/tag';
import { getApiUrl, authenticatedFetch } from '@/config/api.config';
import { useAuth } from '@/auth/AuthContext';
import { Plus, Save, Languages, X, FolderOpen, Tag, Globe } from 'lucide-react';

const AVAILABLE_LANGUAGES = [{ code: 'vi', displayName: 'Vietnamese (vi)' }];

export default function CategoryForm({ id }: { id?: string }) {
  const navigate = useNavigate();
  const { token, keycloak } = useAuth();
  const [fetchingData, setFetchingData] = useState(false);
  const [activeTab, setActiveTab] = useState(0);
  const [fabOpen, setFabOpen] = useState(false);

  const {
    register,
    handleSubmit,
    control,
    reset,
    watch,
    formState: { errors, isSubmitting },
  } = useForm<CategoryFormData>({
    resolver: zodResolver(categoryFormSchema),
    defaultValues: {
      displayName: '',
      categoryType: CategoryTypeEnum.Blog,
      tagNames: [],
      translations: [],
      rowVersion: 0,
    },
  });

  const { fields, append, remove } = useFieldArray({
    control,
    name: 'translations',
  });

  const translations = watch('translations');
  const isLoading = isSubmitting || fetchingData;

  useEffect(() => {
    if (id) {
      const fetchCategory = async () => {
        setFetchingData(true);
        try {
          const response = await authenticatedFetch(
            getApiUrl(`/categories/${id}`),
            token,
            { cache: 'no-store' },
            keycloak || undefined,
          );
          if (response && response.ok) {
            const res: { data: CategoryModel } = await response.json();
            reset({
              displayName: res.data.displayName,
              categoryType: res.data.categoryType,
              tagNames: res.data.tags?.map((tag: TagModel) => tag.name),
              translations: res.data.translations?.map((ct) => ({
                id: ct.id,
                languageCode: ct.languageCode,
                displayName: ct.displayName,
              })),
              rowVersion: res.data.rowVersion,
            });
          } else {
            toast.error('Failed to load category');
          }
        } catch (error) {
          console.error('Failed to load category:', error);
          toast.error('Error loading category');
        } finally {
          setFetchingData(false);
        }
      };

      fetchCategory();
    } else {
      reset({
        displayName: '',
        categoryType: CategoryTypeEnum.Blog,
        tagNames: [],
        translations: [],
        rowVersion: 0,
      });
    }
  }, [id, reset, token, keycloak]);

  const onSubmit = async (data: CategoryFormData) => {
    try {
      if (id) {
        const categoryData: UpdateCategoryModel = {
          id,
          displayName: data.displayName,
          categoryType: data.categoryType,
          tagNames: data.tagNames,
          rowVersion: data.rowVersion,
          translations: data.translations?.map((translation) => ({
            displayName: translation.displayName,
            id: translation.id || undefined,
            languageCode: translation.languageCode,
          })),
        };

        const updateResponse = await authenticatedFetch(
          getApiUrl('/categories'),
          token,
          {
            method: 'PUT',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify(categoryData),
          },
          keycloak || undefined,
        );

        if (updateResponse.ok) {
          toast.success('Category updated successfully');
          navigate('/admin/categories');
        } else {
          const errorData = await updateResponse.json();
          console.error(errorData, updateResponse.status);
          toast.error(errorData.message || 'Failed to update category');
        }
      } else {
        const categoryData: CreateCategoryModel = {
          displayName: data.displayName,
          categoryType: data.categoryType,
          tagNames: data.tagNames,
          translations: data.translations?.map((translation) => ({
            displayName: translation.displayName,
            languageCode: translation.languageCode,
          })),
        };

        const createResponse = await authenticatedFetch(
          getApiUrl('/categories'),
          token,
          {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify(categoryData),
          },
          keycloak || undefined,
        );

        if (createResponse.ok) {
          toast.success('Category created successfully');
          navigate('/admin/categories');
        } else {
          const errorData = await createResponse.json();
          console.error(errorData, createResponse.status);
          toast.error(errorData.message || 'Failed to create category');
        }
      }
    } catch (error) {
      console.error('Error submitting form:', error);
      toast.error('Network error. Please try again.');
    }
  };

  const addTranslationTab = () => {
    append({ id: '', languageCode: '', displayName: '' });
    setActiveTab(fields.length);
  };

  const removeTranslationTab = (index: number) => {
    remove(index);
    if (activeTab >= fields.length - 1) {
      setActiveTab(Math.max(0, fields.length - 2));
    }
  };

  const isAddTabDisabled = () => {
    const usedLanguages = translations?.map((t) => t.languageCode) || [];
    const conditionEveryLanguageCodesUsed = AVAILABLE_LANGUAGES.every((lang) =>
      usedLanguages.includes(lang.code),
    );
    const conditionMaxTabs = fields.length >= AVAILABLE_LANGUAGES.length;
    return conditionEveryLanguageCodesUsed || conditionMaxTabs;
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6 w-full">
      {/* Basic Information Card */}
      <div className="card bg-base-100 shadow-lg border-t-4 border-t-primary hover:shadow-xl transition-shadow duration-300">
        <div className="card-body">
          <div className="flex items-start gap-4">
            <div className="bg-primary/10 p-3 rounded-xl">
              <FolderOpen className="w-6 h-6 text-primary" />
            </div>
            <div className="flex-1">
              <h2 className="card-title text-lg">Basic Information</h2>
              <p className="text-sm text-base-content/60">
                Set the display name and type for this category
              </p>
            </div>
          </div>

          <div className="divider my-2"></div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <label className="form-control w-full">
              <div className="label">
                <span className="label-text font-medium">Display Name</span>
              </div>
              <input
                type="text"
                {...register('displayName')}
                className={`input input-bordered w-full focus:input-primary ${errors.displayName ? 'input-error' : ''}`}
                placeholder="e.g., Technology"
                disabled={isLoading}
              />
              {errors.displayName && (
                <div className="label">
                  <span className="label-text-alt text-error">{errors.displayName.message}</span>
                </div>
              )}
            </label>

            <label className="form-control w-full">
              <div className="label">
                <span className="label-text font-medium">Category Type</span>
              </div>
              <select
                {...register('categoryType')}
                className={`select select-bordered w-full focus:select-primary ${errors.categoryType ? 'select-error' : ''}`}
                disabled={isLoading}
              >
                <option value={CategoryTypeEnum.Blog}>Blog</option>
                <option value={CategoryTypeEnum.Other}>Other</option>
              </select>
              {errors.categoryType && (
                <div className="label">
                  <span className="label-text-alt text-error">{errors.categoryType.message}</span>
                </div>
              )}
            </label>
          </div>
        </div>
      </div>

      {/* Tags Card */}
      <div className="card bg-base-100 shadow-lg border-t-4 border-t-accent hover:shadow-xl transition-shadow duration-300">
        <div className="card-body">
          <div className="flex items-start gap-4">
            <div className="bg-accent/10 p-3 rounded-xl">
              <Tag className="w-6 h-6 text-accent" />
            </div>
            <div className="flex-1">
              <div className="flex items-center justify-between">
                <h2 className="card-title text-lg">Tags</h2>
                <span className="badge badge-accent badge-outline">Optional</span>
              </div>
              <p className="text-sm text-base-content/60">
                Add tags to help organize and filter categories
              </p>
            </div>
          </div>

          <div className="divider my-2"></div>

          <label className="form-control w-full">
            <Controller
              name="tagNames"
              control={control}
              render={({ field }) => (
                <MultiChipInput
                  chips={field.value.map((tag) => ({
                    label: tag,
                    color: getRandomColor(),
                  }))}
                  setChips={(chips: { label: string; color: string }[]) => {
                    field.onChange(chips.map((chip) => chip.label.toLowerCase()));
                  }}
                  className="flex flex-wrap border-2 border-base-300 rounded-xl p-3 min-h-[52px] bg-base-100 focus-within:border-accent transition-colors"
                  loading={isLoading}
                  formControlName="tags"
                />
              )}
            />
            <div className="label">
              <span className="label-text-alt text-base-content/50">Press Enter to add a tag</span>
            </div>
          </label>
        </div>
      </div>

      {/* Translations Card */}
      <div className="card bg-base-100 shadow-lg border-t-4 border-t-secondary hover:shadow-xl transition-shadow duration-300">
        <div className="card-body">
          <div className="flex items-start gap-4">
            <div className="bg-secondary/10 p-3 rounded-xl">
              <Globe className="w-6 h-6 text-secondary" />
            </div>
            <div className="flex-1">
              <div className="flex items-center justify-between">
                <div>
                  <h2 className="card-title text-lg">Translations</h2>
                  <p className="text-sm text-base-content/60">
                    Add translations for different languages
                  </p>
                </div>
                <button
                  type="button"
                  className="btn btn-sm btn-secondary btn-outline gap-1"
                  onClick={addTranslationTab}
                  disabled={isAddTabDisabled() || isLoading}
                >
                  <Plus className="w-4 h-4" />
                  Add Language
                </button>
              </div>
            </div>
          </div>

          <div className="divider my-2"></div>

          {fields.length === 0 ? (
            <div className="text-center py-10 border-2 border-dashed border-base-300 rounded-xl bg-base-200/30">
              <div className="bg-secondary/10 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-3">
                <Languages className="w-8 h-8 text-secondary/50" />
              </div>
              <p className="text-base-content/50 text-sm mb-3">No translations added yet</p>
              <button
                type="button"
                className="btn btn-sm btn-ghost text-secondary"
                onClick={addTranslationTab}
                disabled={isAddTabDisabled() || isLoading}
              >
                <Plus className="w-4 h-4" />
                Add first translation
              </button>
            </div>
          ) : (
            <>
              {/* Translation Tabs */}
              <div className="tabs tabs-boxed bg-base-200 p-1 rounded-xl mb-4">
                {fields.map((field, index) => (
                  <button
                    key={field.id}
                    type="button"
                    className={`tab gap-2 transition-all ${activeTab === index ? 'tab-active bg-secondary text-secondary-content' : ''}`}
                    onClick={() => setActiveTab(index)}
                  >
                    <Globe className="w-3 h-3" />
                    {translations[index]?.languageCode?.toUpperCase() || `New`}
                  </button>
                ))}
              </div>

              {/* Translation Content */}
              {fields.map((field, index) => (
                <div
                  key={field.id}
                  className={`space-y-4 p-4 bg-base-200/30 rounded-xl ${activeTab === index ? '' : 'hidden'}`}
                >
                  <div className="flex items-center justify-between pb-2 border-b border-base-300">
                    <span className="text-sm font-medium text-base-content/70 flex items-center gap-2">
                      <Languages className="w-4 h-4" />
                      Translation #{index + 1}
                    </span>
                    <button
                      type="button"
                      className="btn btn-sm btn-ghost text-error hover:bg-error/10 gap-1"
                      onClick={() => removeTranslationTab(index)}
                      disabled={isLoading}
                    >
                      <X className="w-4 h-4" />
                      Remove
                    </button>
                  </div>

                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <label className="form-control w-full">
                      <div className="label">
                        <span className="label-text font-medium">Language</span>
                      </div>
                      <select
                        {...register(`translations.${index}.languageCode`)}
                        className={`select select-bordered w-full focus:select-secondary ${
                          errors.translations?.[index]?.languageCode ? 'select-error' : ''
                        }`}
                        disabled={isLoading}
                      >
                        <option value="">Select Language</option>
                        {AVAILABLE_LANGUAGES.map((lang) => (
                          <option key={lang.code} value={lang.code}>
                            {lang.displayName}
                          </option>
                        ))}
                      </select>
                      {errors.translations?.[index]?.languageCode && (
                        <div className="label">
                          <span className="label-text-alt text-error">
                            {errors.translations[index]?.languageCode?.message}
                          </span>
                        </div>
                      )}
                    </label>

                    <label className="form-control w-full">
                      <div className="label">
                        <span className="label-text font-medium">Translated Name</span>
                      </div>
                      <input
                        type="text"
                        {...register(`translations.${index}.displayName`)}
                        className={`input input-bordered w-full focus:input-secondary ${
                          errors.translations?.[index]?.displayName ? 'input-error' : ''
                        }`}
                        placeholder="Enter translated name"
                        disabled={isLoading}
                      />
                      {errors.translations?.[index]?.displayName && (
                        <div className="label">
                          <span className="label-text-alt text-error">
                            {errors.translations[index]?.displayName?.message}
                          </span>
                        </div>
                      )}
                    </label>
                  </div>
                </div>
              ))}
            </>
          )}
        </div>
      </div>

      {/* Action Buttons - FAB */}
      <div className="fixed bottom-8 right-8 z-50 flex flex-col items-center gap-3">
        {/* Expandable Buttons */}
        <div className={`flex flex-col items-center gap-3 transition-all duration-300 ${fabOpen ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-4 pointer-events-none'}`}>
          {/* Cancel Button */}
          <button
            type="button"
            className="btn btn-lg btn-circle shadow-md bg-base-100 hover:bg-base-200"
            onClick={() => {
              navigate('/admin/categories');
              setFabOpen(false);
            }}
            disabled={isLoading}
          >
            <X className="w-5 h-5" />
          </button>

          {/* Save/Update Button */}
          <button
            type="submit"
            className="btn btn-lg btn-circle btn-success shadow-md"
            disabled={isLoading}
          >
            {isSubmitting ? (
              <span className="loading loading-spinner loading-sm"></span>
            ) : (
              <Save className="w-5 h-5" />
            )}
          </button>
        </div>

        {/* Main FAB Trigger */}
        <button
          type="button"
          className={`btn btn-lg btn-circle btn-primary shadow-lg transition-transform duration-300 ${fabOpen ? 'rotate-45' : ''}`}
          onClick={() => setFabOpen(!fabOpen)}
        >
          <Plus className="w-5 h-5" />
        </button>
      </div>
    </form>
  );
}
