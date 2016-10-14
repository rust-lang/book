<?xml version="1.0"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w10="urn:schemas-microsoft-com:office:word" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:wp14="http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml">
    <xsl:output method="text" />
    <xsl:template match="/">
        <xsl:apply-templates select="/w:document/w:body/*" />
    </xsl:template>

    <!-- Ignore these -->
    <xsl:template match="w:p[starts-with(w:pPr/w:pStyle/@w:val, 'TOC')]" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'ChapterStart']" />

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'ChapterTitle']">
        <xsl:text>&#10;[TOC]&#10;&#10;</xsl:text>
        <xsl:text># </xsl:text>
        <xsl:value-of select="w:r/w:t" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'HeadA']">
        <xsl:text>## </xsl:text>
        <xsl:value-of select="w:r/w:t" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'BodyFirst' or @w:val = 'Body']]">
        <xsl:apply-templates select="w:r" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:r[w:rPr/w:rStyle/@w:val = 'Literal']">
        <xsl:text>`</xsl:text>
        <xsl:value-of select="w:t" />
        <xsl:text>`</xsl:text>
    </xsl:template>

    <xsl:template match="w:r[w:rPr/w:rStyle/@w:val = 'EmphasisItalic']">
        <xsl:choose>
            <xsl:when test="normalize-space(w:t) != ''">
                <xsl:text>*</xsl:text>
                <xsl:value-of select="w:t" />
                <xsl:text>*</xsl:text>
            </xsl:when>
            <xsl:otherwise>
                <xsl:value-of select="w:t" />
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'CodeA']">
        <xsl:text>```&#10;</xsl:text>
        <xsl:value-of select="w:r/w:t" />
        <xsl:text>&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'CodeB']">
        <xsl:value-of select="w:r/w:t" />
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'CodeC']">
        <xsl:value-of select="w:r/w:t" />
        <xsl:text>&#10;```&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:r">
        <xsl:value-of select="w:t" />
    </xsl:template>

    <xsl:template match="w:p">
Unmatched: <xsl:value-of select="w:pPr/w:pStyle/@w:val" />
      <xsl:text>
      </xsl:text>


    </xsl:template>
</xsl:stylesheet>
