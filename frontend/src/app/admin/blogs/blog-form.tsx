import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useForm, Controller, useFieldArray } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { toast } from 'sonner';
import { blogFormSchema, type BlogFormData } from '@/schemas/blog.schema';
import type { PostModel } from '@/domains/post';
import type { CategoryModel } from '@/domains/category';
import MultiChipInput, {
  getRandomColor,
} from '../components/inputs/multi-chip-input';
import {
  Info,
  ImagePlus,
  Tag,
  BookOpen,
  Save,
  FileText,
  Sparkles,
  Globe,
  Languages,
  Plus,
  X,
  ChevronDown,
  RotateCw,
} from 'lucide-react';
import { RichTextEditor } from '../components/inputs/rich-text-editor/rich-text-editor';
import ThumbnailsInput from '../components/inputs/thumbnail-input';
import { getApiUrl, authenticatedFetch } from '@/config/api.config';
import { useAuth } from '@/auth/AuthContext';
import type { OpenAIModel } from '@/domains/ai-model';

const AVAILABLE_LANGUAGES = [{ code: 'vi', displayName: 'Vietnamese (vi)' }];

export default function BlogForm({ id }: { id?: string }) {
  const navigate = useNavigate();
  const { token, keycloak } = useAuth();
  const [categories, setCategories] = useState<CategoryModel[]>([]);
  const [originalContent, setOriginalContent] = useState('');
  const [originalTranslationContents, setOriginalTranslationContents] = useState<
    Record<number, string>
  >({});
  const [fetchingData, setFetchingData] = useState(false);
  const [activeTranslationTab, setActiveTranslationTab] = useState(0);
  const [activeMainTab, setActiveMainTab] = useState<'main' | 'translations'>('main');

  // Collapsible states for main content sections
  const [basicInfoOpen, setBasicInfoOpen] = useState(true);
  const [thumbnailsOpen, setThumbnailsOpen] = useState(true);
  const [tagsOpen, setTagsOpen] = useState(true);
  const [previewContentOpen, setPreviewContentOpen] = useState(true);
  const [fullArticleOpen, setFullArticleOpen] = useState(true);
  const [fabOpen, setFabOpen] = useState(false);
  
  // Translation modal state
  const [isTranslating, setIsTranslating] = useState(false);
  const [showTranslateModal, setShowTranslateModal] = useState(false);
  const [selectedTranslateLanguage, setSelectedTranslateLanguage] = useState('');
  const [retranslatingIndex, setRetranslatingIndex] = useState<number | null>(null);
  
  // AI model selection state
  const [aiModels, setAiModels] = useState<OpenAIModel[]>([]);
  const [selectedModel, setSelectedModel] = useState<string>('gpt-5-nano');
  const [loadingModels, setLoadingModels] = useState(false);

  const {
    register,
    handleSubmit,
    control,
    reset,
    watch,
    setValue,
    formState: { errors, isSubmitting },
  } = useForm<BlogFormData>({
    resolver: zodResolver(blogFormSchema),
    defaultValues: {
      title: '',
      previewContent: '',
      content: '',
      thumbnailPaths: [],
      published: false,
      tagNames: [],
      categoryId: '',
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

  // Reusable function to reload post data
  const reloadPostData = async () => {
    if (!id) return;
    
    setFetchingData(true);
    try {
      const response = await authenticatedFetch(
        getApiUrl(`/posts/${id}`),
        token,
        { cache: 'no-store' },
        keycloak || undefined
      );
      
      if (response && response.ok) {
        const res: { data: PostModel } = await response.json();
        setOriginalContent(res.data.content);

        // Store original translation contents for rich text editors
        const translationContents: Record<number, string> = {};
        res.data.translations?.forEach((t, index) => {
          translationContents[index] = t.content;
        });
        setOriginalTranslationContents(translationContents);

        reset({
          title: res.data.title,
          previewContent: res.data.previewContent,
          content: res.data.content,
          thumbnailPaths: res.data.thumbnailPaths ?? [],
          published: res.data.published,
          tagNames: res.data.tags?.map((tag) => tag.name) ?? [],
          categoryId: res.data.categoryId,
          translations:
            res.data.translations?.map((t) => ({
              id: t.id,
              languageCode: t.languageCode,
              title: t.title,
              previewContent: t.previewContent,
              content: t.content,
              slug: t.slug,
            })) ?? [],
          rowVersion: res.data.rowVersion,
        });
      } else {
        toast.error('Failed to load post data');
      }
    } catch (error) {
      console.error('Error fetching post:', error);
      toast.error('Error loading post');
    } finally {
      setFetchingData(false);
    }
  };

  // Fetch AI models
  const fetchAIModels = async () => {
    setLoadingModels(true);
    try {
      const response = await authenticatedFetch(
        getApiUrl('/ai/models'),
        token,
        { cache: 'no-store' },
        keycloak || undefined
      );
      if (response.ok) {
        const result = await response.json();
        setAiModels(result.data.models);
      }
    } catch (error) {
      console.error('Error fetching AI models:', error);
    } finally {
      setLoadingModels(false);
    }
  };

  useEffect(() => {
    if (id) {
      reloadPostData();
    } else {
      reset({
        title: '',
        previewContent: '',
        content: '',
        thumbnailPaths: [],
        published: false,
        tagNames: [],
        categoryId: '',
        translations: [],
        rowVersion: 0,
      });
      setOriginalContent('');
      setOriginalTranslationContents({});
    }
  }, [id, reset, token, keycloak]);

  // Fetch AI models when modal is opened
  useEffect(() => {
    if (showTranslateModal && aiModels.length === 0) {
      fetchAIModels();
    }
  }, [showTranslateModal]);

  useEffect(() => {
    const fetchCategories = async () => {
      try {
        const response = await authenticatedFetch(
          getApiUrl('/categories'),
          token,
          { cache: 'no-store' },
          keycloak || undefined
        );
        if (response.ok) {
          const res: { data: CategoryModel[] } = await response.json();
          setCategories(res.data);

          const currentCategoryId = watch('categoryId');
          if (!currentCategoryId && res.data.length > 0 && !id) {
            setValue('categoryId', res.data[0].id);
          }
        }
      } catch (error) {
        console.error('Error fetching categories:', error);
        toast.error('Failed to load categories');
        setCategories([]);
      }
    };

    fetchCategories();
  }, [id, token, keycloak, setValue, watch]);

  const onSubmit = async (data: BlogFormData) => {
    try {
      const postData = {
        id,
        ...data,
        translations: data.translations?.map((t) => ({
          id: t.id || undefined,
          languageCode: t.languageCode,
          title: t.title,
          previewContent: t.previewContent,
          content: t.content,
          slug: t.slug || undefined,
        })),
      };

      const method = id ? 'PUT' : 'POST';

      const response = await authenticatedFetch(
        getApiUrl('/posts'),
        token,
        {
          method,
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(postData),
        },
        keycloak || undefined
      );

      if (response.ok) {
        toast.success(id ? 'Post updated successfully' : 'Post created successfully');
        navigate('/admin/blogs');
      } else {
        const errorData = await response.json();
        console.error(errorData, response.status);
        toast.error(errorData.message || 'Failed to save post');
      }
    } catch (error) {
      console.error('Error submitting form:', error);
      toast.error('Network error. Please try again.');
    }
  };

  const onFormError = (formErrors: typeof errors) => {
    // Collect all error messages
    const errorMessages: string[] = [];

    const extractErrors = (obj: Record<string, unknown>, prefix = '') => {
      for (const [key, value] of Object.entries(obj)) {
        if (value && typeof value === 'object') {
          if ('message' in value && typeof value.message === 'string') {
            const fieldName = prefix ? `${prefix}.${key}` : key;
            errorMessages.push(`${fieldName}: ${value.message}`);
          } else if (Array.isArray(value)) {
            value.forEach((item, index) => {
              if (item && typeof item === 'object') {
                extractErrors(item as Record<string, unknown>, `${key}[${index}]`);
              }
            });
          } else {
            extractErrors(value as Record<string, unknown>, prefix ? `${prefix}.${key}` : key);
          }
        }
      }
    };

    extractErrors(formErrors as Record<string, unknown>);

    if (errorMessages.length > 0) {
      // Show first error as main toast, mention if there are more
      const firstError = errorMessages[0];
      const moreCount = errorMessages.length - 1;
      const message = moreCount > 0
        ? `${firstError} (+${moreCount} more errors)`
        : firstError;
      toast.error(message);
    } else {
      toast.error('Please fix the form errors before submitting');
    }
  };

  const addTranslationTab = () => {
    append({
      id: '',
      languageCode: '',
      title: '',
      previewContent: '',
      content: '',
      slug: '',
    });
    setActiveTranslationTab(fields.length);
  };

  const removeTranslationTab = (index: number) => {
    remove(index);
    // Update original contents mapping
    const newContents: Record<number, string> = {};
    Object.entries(originalTranslationContents).forEach(([key, value]) => {
      const keyNum = parseInt(key);
      if (keyNum < index) {
        newContents[keyNum] = value;
      } else if (keyNum > index) {
        newContents[keyNum - 1] = value;
      }
    });
    setOriginalTranslationContents(newContents);

    if (activeTranslationTab >= fields.length - 1) {
      setActiveTranslationTab(Math.max(0, fields.length - 2));
    }
  };

  const isAddTranslationDisabled = () => {
    const usedLanguages = translations?.map((t) => t.languageCode) || [];
    const allUsed = AVAILABLE_LANGUAGES.every((lang) => usedLanguages.includes(lang.code));
    const maxReached = fields.length >= AVAILABLE_LANGUAGES.length;
    return allUsed || maxReached;
  };

  const handleTranslatePost = async () => {
    if (!id || !selectedTranslateLanguage) {
      toast.error('Please select a language to translate to');
      return;
    }

    setIsTranslating(true);
    try {
      const response = await authenticatedFetch(
        getApiUrl(`/posts/${id}/translate`),
        token,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            targetLanguage: selectedTranslateLanguage,
            forceRetranslate: false,
            model: selectedModel,
          }),
        },
        keycloak || undefined
      );

      if (response.ok) {
        await response.json();
        toast.success('Translation completed successfully!');
        setShowTranslateModal(false);
        setSelectedTranslateLanguage('');
        
        // Reload post data to show the new translation
        await reloadPostData();
      } else {
        let errorMessage = 'Failed to translate post';
        try {
          const errorData = await response.json();
          if (errorData.message) {
            errorMessage = errorData.message;
          }
        } catch {
          // If parsing JSON fails, use default error message
        }
        toast.error(errorMessage);
      }
    } catch (error) {
      console.error('Error translating post:', error);
      toast.error('Network error. Please try again.');
    } finally {
      setIsTranslating(false);
    }
  };

  const handleRetranslateTranslation = async (index: number) => {
    if (!id) {
      toast.error('Post ID is required to re-translate');
      return;
    }

    const translation = translations[index];
    if (!translation || !translation.languageCode) {
      toast.error('Please select a language for this translation');
      return;
    }

    setRetranslatingIndex(index);
    try {
      const response = await authenticatedFetch(
        getApiUrl(`/posts/${id}/translate`),
        token,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            targetLanguage: translation.languageCode,
            forceRetranslate: true,
            model: selectedModel,
          }),
        },
        keycloak || undefined
      );

      if (response.ok) {
        await response.json();
        toast.success(`Translation re-generated successfully for ${translation.languageCode.toUpperCase()}!`);
        
        // Reload post data to show the updated translation
        await reloadPostData();
      } else {
        let errorMessage = 'Failed to re-translate';
        try {
          const errorData = await response.json();
          if (errorData.message) {
            errorMessage = errorData.message;
          }
        } catch {
          // If parsing JSON fails, use default error message
        }
        toast.error(errorMessage);
      }
    } catch (error) {
      console.error('Error re-translating:', error);
      toast.error('Network error. Please try again.');
    } finally {
      setRetranslatingIndex(null);
    }
  };

  const isRetranslateDisabled = (index: number) => {
    return isLoading || retranslatingIndex === index || !translations[index]?.languageCode;
  };

  const getAvailableTranslationLanguages = () => {
    const usedLanguages = translations?.map((t) => t.languageCode) || [];
    return AVAILABLE_LANGUAGES.filter((lang) => !usedLanguages.includes(lang.code));
  };

  return (
    <form onSubmit={handleSubmit(onSubmit, onFormError)} className="space-y-6 w-full">
      {/* Main Tabs - Modern Segmented Control */}
      <div className="bg-base-200/50 p-1.5 rounded-2xl inline-flex gap-1 shadow-inner">
        <button
          type="button"
          className={`relative px-6 py-3 rounded-xl font-medium text-sm transition-all duration-300 flex items-center gap-2
            ${activeMainTab === 'main'
              ? 'bg-white text-primary shadow-md shadow-primary/10 scale-[1.02]'
              : 'text-base-content/60 hover:text-base-content hover:bg-base-100/50'
            }`}
          onClick={() => setActiveMainTab('main')}
        >
          <div className={`p-1.5 rounded-lg transition-colors duration-300 ${activeMainTab === 'main' ? 'bg-primary/10' : 'bg-transparent'}`}>
            <FileText className={`w-4 h-4 transition-colors duration-300 ${activeMainTab === 'main' ? 'text-primary' : ''}`} />
          </div>
          <span>Main Content</span>
          {activeMainTab === 'main' && (
            <span className="absolute -bottom-1 left-1/2 -translate-x-1/2 w-8 h-1 bg-primary rounded-full" />
          )}
        </button>
        <button
          type="button"
          className={`relative px-6 py-3 rounded-xl font-medium text-sm transition-all duration-300 flex items-center gap-2
            ${activeMainTab === 'translations'
              ? 'bg-white text-neutral shadow-md shadow-neutral/10 scale-[1.02]'
              : 'text-base-content/60 hover:text-base-content hover:bg-base-100/50'
            }`}
          onClick={() => setActiveMainTab('translations')}
        >
          <div className={`p-1.5 rounded-lg transition-colors duration-300 ${activeMainTab === 'translations' ? 'bg-neutral/10' : 'bg-transparent'}`}>
            <Globe className={`w-4 h-4 transition-colors duration-300 ${activeMainTab === 'translations' ? 'text-neutral' : ''}`} />
          </div>
          <span>Translations</span>
          {fields.length > 0 && (
            <span className={`badge badge-sm transition-all duration-300 ${activeMainTab === 'translations' ? 'badge-neutral' : 'badge-ghost'}`}>
              {fields.length}
            </span>
          )}
          {activeMainTab === 'translations' && (
            <span className="absolute -bottom-1 left-1/2 -translate-x-1/2 w-8 h-1 bg-neutral rounded-full" />
          )}
        </button>
      </div>

      {/* Tab Content */}
      <div className="bg-base-100 rounded-2xl border border-base-300 p-6 shadow-sm">
        {/* Main Content Tab */}
        {activeMainTab === 'main' && (
          <div className="space-y-4">
            {/* Basic Information - Collapsible */}
            <div className="collapse bg-base-100 border border-base-300 rounded-xl">
              <input
                type="checkbox"
                checked={basicInfoOpen}
                onChange={(e) => setBasicInfoOpen(e.target.checked)}
              />
              <div className="collapse-title font-medium flex items-center gap-3">
                <div className="bg-primary/10 p-2 rounded-lg">
                  <Info className="w-5 h-5 text-primary" />
                </div>
                <span>Basic Information</span>
                <ChevronDown className={`w-4 h-4 ml-auto transition-transform duration-300 ${basicInfoOpen ? 'rotate-180' : ''}`} />
              </div>
              <div className="collapse-content">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
                  <label className="form-control w-full md:col-span-2">
                    <div className="label">
                      <span className="label-text font-medium">Title</span>
                    </div>
                    <input
                      type="text"
                      {...register('title')}
                      className={`input input-bordered w-full focus:input-primary ${errors.title ? 'input-error' : ''}`}
                      placeholder="Enter an engaging title"
                      disabled={isLoading}
                    />
                    {errors.title && (
                      <div className="label">
                        <span className="label-text-alt text-error">{errors.title.message}</span>
                      </div>
                    )}
                  </label>

                  <label className="form-control w-full">
                    <div className="label">
                      <span className="label-text font-medium">Category</span>
                    </div>
                    <select
                      {...register('categoryId')}
                      className={`select select-bordered w-full focus:select-primary ${errors.categoryId ? 'select-error' : ''}`}
                      disabled={isLoading || categories.length === 0}
                    >
                      <option value="" disabled>
                        Select a category
                      </option>
                      {categories.map((category) => (
                        <option key={category.id} value={category.id}>
                          {category.displayName}
                        </option>
                      ))}
                    </select>
                    {errors.categoryId && (
                      <div className="label">
                        <span className="label-text-alt text-error">{errors.categoryId.message}</span>
                      </div>
                    )}
                    {categories.length === 0 && (
                      <div className="label">
                        <span className="label-text-alt text-warning">No categories available</span>
                      </div>
                    )}
                  </label>

                  <div className="form-control w-full">
                    <div className="label">
                      <span className="label-text font-medium">Published Status</span>
                    </div>
                    <Controller
                      name="published"
                      control={control}
                      render={({ field }) => (
                        <div
                          className={`flex items-center gap-3 border-2 rounded-xl px-4 h-12 transition-all duration-200 ${
                            field.value ? 'border-success bg-success/5' : 'border-base-300 bg-base-100'
                          }`}
                        >
                          <input
                            type="checkbox"
                            className={`toggle ${field.value ? 'toggle-success' : ''}`}
                            checked={field.value}
                            onChange={field.onChange}
                            disabled={isLoading}
                          />
                          <span
                            className={`font-medium ${field.value ? 'text-success' : 'text-base-content/60'}`}
                          >
                            {field.value ? 'Published' : 'Draft'}
                          </span>
                          {field.value && <Sparkles className="w-4 h-4 text-success ml-auto" />}
                        </div>
                      )}
                    />
                  </div>
                </div>
              </div>
            </div>

            {/* Thumbnails - Collapsible */}
            <div className="collapse bg-base-100 border border-base-300 rounded-xl">
              <input
                type="checkbox"
                checked={thumbnailsOpen}
                onChange={(e) => setThumbnailsOpen(e.target.checked)}
              />
              <div className="collapse-title font-medium flex items-center gap-3">
                <div className="bg-secondary/10 p-2 rounded-lg">
                  <ImagePlus className="w-5 h-5 text-secondary" />
                </div>
                <span>Thumbnails</span>
                <span className="badge badge-secondary badge-outline badge-sm ml-2">Optional</span>
                <ChevronDown className={`w-4 h-4 ml-auto transition-transform duration-300 ${thumbnailsOpen ? 'rotate-180' : ''}`} />
              </div>
              <div className="collapse-content">
                <div className="pt-2">
                  <Controller
                    name="thumbnailPaths"
                    control={control}
                    render={({ field }) => (
                      <ThumbnailsInput
                        value={field.value}
                        onUploadSuccess={(urls) => field.onChange([...urls])}
                      />
                    )}
                  />
                </div>
              </div>
            </div>

            {/* Tags - Collapsible */}
            <div className="collapse bg-base-100 border border-base-300 rounded-xl">
              <input
                type="checkbox"
                checked={tagsOpen}
                onChange={(e) => setTagsOpen(e.target.checked)}
              />
              <div className="collapse-title font-medium flex items-center gap-3">
                <div className="bg-accent/10 p-2 rounded-lg">
                  <Tag className="w-5 h-5 text-accent" />
                </div>
                <span>Tags</span>
                <span className="badge badge-accent badge-outline badge-sm ml-2">Optional</span>
                <ChevronDown className={`w-4 h-4 ml-auto transition-transform duration-300 ${tagsOpen ? 'rotate-180' : ''}`} />
              </div>
              <div className="collapse-content">
                <div className="pt-2">
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
                </div>
              </div>
            </div>

            {/* Preview Content - Collapsible */}
            <div className="collapse bg-base-100 border border-base-300 rounded-xl">
              <input
                type="checkbox"
                checked={previewContentOpen}
                onChange={(e) => setPreviewContentOpen(e.target.checked)}
              />
              <div className="collapse-title font-medium flex items-center gap-3">
                <div className="bg-info/10 p-2 rounded-lg">
                  <BookOpen className="w-5 h-5 text-info" />
                </div>
                <span>Preview Content</span>
                <ChevronDown className={`w-4 h-4 ml-auto transition-transform duration-300 ${previewContentOpen ? 'rotate-180' : ''}`} />
              </div>
              <div className="collapse-content">
                <div className="pt-2">
                  <textarea
                    {...register('previewContent')}
                    className={`textarea textarea-bordered w-full min-h-28 focus:textarea-info ${errors.previewContent ? 'textarea-error' : ''}`}
                    placeholder="Enter a brief preview of your blog post..."
                    disabled={isLoading}
                  />
                  {errors.previewContent && (
                    <div className="label">
                      <span className="label-text-alt text-error">{errors.previewContent.message}</span>
                    </div>
                  )}
                </div>
              </div>
            </div>

            {/* Full Article Content - Collapsible */}
            <div className="collapse bg-base-100 border border-base-300 rounded-xl">
              <input
                type="checkbox"
                checked={fullArticleOpen}
                onChange={(e) => setFullArticleOpen(e.target.checked)}
              />
              <div className="collapse-title font-medium flex items-center gap-3">
                <div className="bg-warning/10 p-2 rounded-lg">
                  <FileText className="w-5 h-5 text-warning" />
                </div>
                <span>Full Article Content</span>
                <ChevronDown className={`w-4 h-4 ml-auto transition-transform duration-300 ${fullArticleOpen ? 'rotate-180' : ''}`} />
              </div>
              <div className="collapse-content">
                <div className="pt-2">
                  <div
                    className="form-control w-full bg-base-100 rounded-xl border-2 border-base-300 overflow-hidden"
                    key="main-editor"
                  >
                    <Controller
                      name="content"
                      control={control}
                      render={({ field }) => (
                        <RichTextEditor
                          key={`editor-${id}`}
                          id="content-editor"
                          defaultValue={originalContent}
                          onTextChange={(value: string) => {
                            field.onChange(value);
                          }}
                          onSelectionChange={() => {}}
                          readOnly={false}
                        />
                      )}
                    />
                  </div>
                  {errors.content && (
                    <div className="label">
                      <span className="label-text-alt text-error">{errors.content.message}</span>
                    </div>
                  )}
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Translations Tab */}
        {activeMainTab === 'translations' && (
          <div className="space-y-4">
            <div className="flex justify-end gap-2">
              {id && getAvailableTranslationLanguages().length > 0 && (
                <button
                  type="button"
                  className="btn btn-sm btn-primary gap-1"
                  onClick={() => setShowTranslateModal(true)}
                  disabled={isLoading || isTranslating}
                >
                  <Sparkles className="w-4 h-4" />
                  AI Translate
                </button>
              )}
              <button
                type="button"
                className="btn btn-sm btn-neutral btn-outline gap-1"
                onClick={addTranslationTab}
                disabled={isAddTranslationDisabled() || isLoading}
              >
                <Plus className="w-4 h-4" />
                Add Language
              </button>
            </div>

            {fields.length === 0 ? (
              <div className="text-center py-10 border-2 border-dashed border-base-300 rounded-xl bg-base-200/30">
                <div className="bg-neutral/10 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-3">
                  <Languages className="w-8 h-8 text-neutral/50" />
                </div>
                <p className="text-base-content/50 text-sm mb-3">No translations added yet</p>
                <button
                  type="button"
                  className="btn btn-sm btn-ghost text-neutral"
                  onClick={addTranslationTab}
                  disabled={isAddTranslationDisabled() || isLoading}
                >
                  <Plus className="w-4 h-4" />
                  Add first translation
                </button>
              </div>
            ) : (
              <>
                {/* Translation Language Tabs */}
                <div className="flex flex-wrap gap-2 mb-4">
                  {fields.map((field, index) => (
                    <button
                      key={field.id}
                      type="button"
                      className={`relative px-4 py-2 rounded-xl font-medium text-sm transition-all duration-300 flex items-center gap-2
                        ${activeTranslationTab === index
                          ? 'bg-neutral text-neutral-content shadow-md shadow-neutral/20'
                          : 'bg-base-200 text-base-content/60 hover:bg-base-300 hover:text-base-content'
                        }`}
                      onClick={() => setActiveTranslationTab(index)}
                    >
                      <Globe className="w-3.5 h-3.5" />
                      <span>{translations[index]?.languageCode?.toUpperCase() || 'New'}</span>
                      {activeTranslationTab === index && (
                        <span className="absolute -bottom-1 left-1/2 -translate-x-1/2 w-6 h-0.5 bg-neutral-content/50 rounded-full" />
                      )}
                    </button>
                  ))}
                </div>

                {/* Translation Content */}
                {fields.map((field, index) => (
                  <div
                    key={field.id}
                    className={`space-y-4 ${activeTranslationTab === index ? '' : 'hidden'}`}
                  >
                    <div className="flex items-center justify-between p-3 bg-base-200/50 rounded-xl">
                      <span className="text-sm font-medium text-base-content/70 flex items-center gap-2">
                        <Languages className="w-4 h-4" />
                        Translation #{index + 1}
                      </span>
                      <div className="flex items-center gap-2">
                        <button
                          type="button"
                          className="btn btn-sm btn-ghost text-primary hover:bg-primary/10 gap-1"
                          onClick={() => handleRetranslateTranslation(index)}
                          disabled={isRetranslateDisabled(index)}
                          title="Re-translate this translation using AI"
                        >
                          {retranslatingIndex === index ? (
                            <>
                              <span className="loading loading-spinner loading-xs"></span>
                              Re-translating...
                            </>
                          ) : (
                            <>
                              <RotateCw className="w-4 h-4" />
                              Re-translate
                            </>
                          )}
                        </button>
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
                    </div>

                    {/* Language Selection */}
                    <label className="form-control w-full">
                      <div className="label">
                        <span className="label-text font-medium">Language</span>
                      </div>
                      <select
                        {...register(`translations.${index}.languageCode`)}
                        className={`select select-bordered w-full focus:select-neutral ${
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

                    {/* Translated Title */}
                    <label className="form-control w-full">
                      <div className="label">
                        <span className="label-text font-medium">Translated Title</span>
                      </div>
                      <input
                        type="text"
                        {...register(`translations.${index}.title`)}
                        className={`input input-bordered w-full focus:input-neutral ${
                          errors.translations?.[index]?.title ? 'input-error' : ''
                        }`}
                        placeholder="Enter translated title"
                        disabled={isLoading}
                      />
                      {errors.translations?.[index]?.title && (
                        <div className="label">
                          <span className="label-text-alt text-error">
                            {errors.translations[index]?.title?.message}
                          </span>
                        </div>
                      )}
                    </label>

                    {/* Translated Preview Content */}
                    <label className="form-control w-full">
                      <div className="label">
                        <span className="label-text font-medium">Translated Preview</span>
                      </div>
                      <textarea
                        {...register(`translations.${index}.previewContent`)}
                        className={`textarea textarea-bordered w-full min-h-24 focus:textarea-neutral ${
                          errors.translations?.[index]?.previewContent ? 'textarea-error' : ''
                        }`}
                        placeholder="Enter translated preview content"
                        disabled={isLoading}
                      />
                      {errors.translations?.[index]?.previewContent && (
                        <div className="label">
                          <span className="label-text-alt text-error">
                            {errors.translations[index]?.previewContent?.message}
                          </span>
                        </div>
                      )}
                    </label>

                    {/* Translated Full Content */}
                    <div className="form-control w-full">
                      <div className="label">
                        <span className="label-text font-medium">Translated Full Content</span>
                      </div>
                      <div className="bg-base-100 rounded-xl border-2 border-base-300 overflow-hidden">
                        <Controller
                          name={`translations.${index}.content`}
                          control={control}
                          render={({ field: contentField }) => (
                            <RichTextEditor
                              key={`translation-editor-${index}-${id}`}
                              id={`translation-content-editor-${index}`}
                              defaultValue={originalTranslationContents[index] || ''}
                              onTextChange={(value: string) => {
                                contentField.onChange(value);
                              }}
                              onSelectionChange={() => {}}
                              readOnly={false}
                            />
                          )}
                        />
                      </div>
                      {errors.translations?.[index]?.content && (
                        <div className="label">
                          <span className="label-text-alt text-error">
                            {errors.translations[index]?.content?.message}
                          </span>
                        </div>
                      )}
                    </div>
                  </div>
                ))}
              </>
            )}
          </div>
        )}
      </div>

      {/* Action Buttons - Custom FAB with toggle */}
      <div className="fixed bottom-8 right-8 z-50 flex flex-col items-center gap-3">
        {/* Expandable Buttons */}
        <div className={`flex flex-col items-center gap-3 transition-all duration-300 ${fabOpen ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-4 pointer-events-none'}`}>
          {/* Cancel Button */}
          <button
            type="button"
            className="btn btn-lg btn-circle shadow-md bg-base-100 hover:bg-base-200"
            onClick={() => {
              navigate('/admin/blogs');
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

      {/* Translation Modal */}
      {showTranslateModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
          <div className="bg-base-100 rounded-2xl shadow-xl max-w-md w-full mx-4 p-6 space-y-4 animate-in fade-in zoom-in duration-200">
            <div className="flex items-center gap-3">
              <div className="bg-primary/10 p-3 rounded-xl">
                <Sparkles className="w-6 h-6 text-primary" />
              </div>
              <div>
                <h3 className="text-lg font-bold">AI Translation</h3>
                <p className="text-sm text-base-content/60">Translate this post automatically</p>
              </div>
            </div>

            {isTranslating ? (
              <div className="py-8 space-y-4">
                <div className="flex flex-col items-center gap-4">
                  <div className="relative">
                    <div className="w-16 h-16 border-4 border-primary/20 border-t-primary rounded-full animate-spin"></div>
                    <Sparkles className="w-6 h-6 text-primary absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" />
                  </div>
                  <div className="text-center space-y-2">
                    <p className="font-medium">Translating your post...</p>
                    <p className="text-sm text-base-content/60">This may take a moment</p>
                  </div>
                </div>
                <div 
                  className="w-full bg-base-200 rounded-full h-2 overflow-hidden" 
                  role="progressbar" 
                  aria-label="Translation in progress"
                  aria-valuemin={0}
                  aria-valuemax={100}
                >
                  <div className="h-full bg-gradient-to-r from-primary via-secondary to-accent animate-pulse"></div>
                  <span className="sr-only">Translating post content, please wait...</span>
                </div>
              </div>
            ) : (
              <>
                <div className="form-control w-full">
                  <label className="label">
                    <span className="label-text font-medium">Target Language</span>
                  </label>
                  <select
                    className="select select-bordered w-full focus:select-primary"
                    value={selectedTranslateLanguage}
                    onChange={(e) => setSelectedTranslateLanguage(e.target.value)}
                  >
                    <option value="">Select a language</option>
                    {getAvailableTranslationLanguages().map((lang) => (
                      <option key={lang.code} value={lang.code}>
                        {lang.displayName}
                      </option>
                    ))}
                  </select>
                  <label className="label">
                    <span className="label-text-alt text-base-content/60">
                      AI will translate the title, preview, and content
                    </span>
                  </label>
                </div>

                <div className="form-control w-full">
                  <label className="label">
                    <span className="label-text font-medium">AI Model</span>
                  </label>
                  {loadingModels ? (
                    <div className="flex items-center justify-center p-4">
                      <span className="loading loading-spinner loading-sm"></span>
                      <span className="ml-2 text-sm">Loading models...</span>
                    </div>
                  ) : (
                    <>
                      <select
                        className="select select-bordered w-full focus:select-primary"
                        value={selectedModel}
                        onChange={(e) => setSelectedModel(e.target.value)}
                      >
                        {aiModels.map((model) => (
                          <option key={model.id} value={model.id}>
                            {model.name} - ${model.inputPricePer1m.toFixed(2)}/${model.outputPricePer1m.toFixed(2)} per 1M tokens
                            {model.isRecommended ? ' ⭐' : ''}
                          </option>
                        ))}
                      </select>
                      <label className="label">
                        <span className="label-text-alt text-base-content/60">
                          {(() => {
                            const currentModel = aiModels.find(m => m.id === selectedModel);
                            if (!currentModel) return null;
                            
                            if (currentModel.isRecommended) {
                              return (
                                <span className="text-warning font-medium">
                                  ⭐ Recommended: {currentModel.recommendationReason}
                                </span>
                              );
                            }
                            
                            return (
                              <span>
                                Input: ${currentModel.inputPricePer1m.toFixed(2)}/1M tokens, 
                                Output: ${currentModel.outputPricePer1m.toFixed(2)}/1M tokens
                              </span>
                            );
                          })()}
                        </span>
                      </label>
                    </>
                  )}
                </div>

                <div className="flex gap-2 justify-end pt-2">
                  <button
                    type="button"
                    className="btn btn-ghost"
                    onClick={() => {
                      setShowTranslateModal(false);
                      setSelectedTranslateLanguage('');
                    }}
                  >
                    Cancel
                  </button>
                  <button
                    type="button"
                    className="btn btn-primary gap-2"
                    onClick={handleTranslatePost}
                    disabled={!selectedTranslateLanguage}
                  >
                    <Sparkles className="w-4 h-4" />
                    Start Translation
                  </button>
                </div>
              </>
            )}
          </div>
        </div>
      )}
    </form>
  );
}
