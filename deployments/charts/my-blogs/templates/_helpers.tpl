{{/*
Expand the name of the chart.
*/}}
{{- define "my-blogs.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "my-blogs.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "my-blogs.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
=====================================================
Admin Side Helpers
=====================================================
*/}}

{{- define "my-blogs.adminSide.name" -}}
{{- printf "%s-admin-side" (include "my-blogs.name" .) | trunc 63 | trimSuffix "-" }}
{{- end }}

{{- define "my-blogs.adminSide.fullname" -}}
{{- printf "%s-admin-side" (include "my-blogs.fullname" .) | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Admin Side Common labels
*/}}
{{- define "my-blogs.adminSide.labels" -}}
helm.sh/chart: {{ include "my-blogs.chart" . }}
{{ include "my-blogs.adminSide.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
app.kubernetes.io/component: admin-side
{{- end }}

{{/*
Admin Side Selector labels
*/}}
{{- define "my-blogs.adminSide.selectorLabels" -}}
app.kubernetes.io/name: {{ include "my-blogs.adminSide.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
=====================================================
Client Side Helpers
=====================================================
*/}}

{{- define "my-blogs.clientSide.name" -}}
{{- printf "%s-client-side" (include "my-blogs.name" .) | trunc 63 | trimSuffix "-" }}
{{- end }}

{{- define "my-blogs.clientSide.fullname" -}}
{{- printf "%s-client-side" (include "my-blogs.fullname" .) | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Client Side Common labels
*/}}
{{- define "my-blogs.clientSide.labels" -}}
helm.sh/chart: {{ include "my-blogs.chart" . }}
{{ include "my-blogs.clientSide.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
app.kubernetes.io/component: client-side
{{- end }}

{{/*
Client Side Selector labels
*/}}
{{- define "my-blogs.clientSide.selectorLabels" -}}
app.kubernetes.io/name: {{ include "my-blogs.clientSide.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
=====================================================
Shared Helpers
=====================================================
*/}}

{{/*
Create the image pull secret
*/}}
{{- define "my-blogs.imagePullSecret" }}
{{- with .Values.imageCredentials }}
{{- printf "{\"auths\":{\"%s\":{\"username\":\"%s\",\"password\":\"%s\",\"email\":\"%s\",\"auth\":\"%s\"}}}" .registry .username .password .email (printf "%s:%s" .username .password | b64enc) | b64enc }}
{{- end }}
{{- end }}
