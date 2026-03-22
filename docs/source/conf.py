# Configuration file for the Sphinx documentation builder.

project = 'agently Edge'
copyright = '2026, agently Team'
author = 'agently Team'
release = '0.1.0'

extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.napoleon',
    'sphinx.ext.viewcode',
    'sphinx.ext.githubpages',
]

templates_path = ['_templates']
exclude_patterns = []

html_theme = 'sphinx_rtd_theme'
html_static_path = ['_static']

# GitHub Pages
html_baseurl = 'https://agently-top.github.io/agently-edge/'
