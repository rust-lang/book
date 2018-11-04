<?xml version="1.0"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w10="urn:schemas-microsoft-com:office:word" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:wp14="http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml">
    <xsl:output method="text" />
    <xsl:template match="/">
        <xsl:apply-templates select="/w:document/w:body/*" />
    </xsl:template>

    <!-- Ignore these -->
    <xsl:template match="w:p[starts-with(w:pPr/w:pStyle/@w:val, 'TOC')]" />
    <xsl:template match="w:p[starts-with(w:pPr/w:pStyle/@w:val, 'Contents1')]" />
    <xsl:template match="w:p[starts-with(w:pPr/w:pStyle/@w:val, 'Contents2')]" />
    <xsl:template match="w:p[starts-with(w:pPr/w:pStyle/@w:val, 'Contents3')]" />

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'ChapterStart']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'Normal']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'Standard']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'AuthorQuery']" />

    <!-- Paragraph styles -->

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'ChapterTitle']">
        <xsl:text>&#10;[TOC]&#10;&#10;</xsl:text>
        <xsl:text># </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'HeadA']">
        <xsl:text>## </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'HeadB']">
        <xsl:text>### </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'HeadC']">
        <xsl:text>#### </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'HeadBox']">
        <xsl:text>### </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'NumListA' or @w:val = 'NumListB']]">
        <xsl:text>1. </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'NumListC']]">
        <xsl:text>1. </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'BulletA' or @w:val = 'BulletB' or @w:val = 'ListPlainA' or @w:val = 'ListPlainB']]">
        <xsl:text>* </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'BulletC' or @w:val = 'ListPlainC']]">
        <xsl:text>* </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'SubBullet']]">
        <xsl:text>  * </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'BodyFirst' or @w:val = 'Body' or @w:val = 'BodyFirstBox' or @w:val = 'BodyBox' or @w:val = '1stPara']]">
        <xsl:if test=".//w:t">
            <xsl:apply-templates select="*" />
            <xsl:text>&#10;&#10;</xsl:text>
        </xsl:if>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'CodeA' or @w:val = 'CodeAWingding']]">
        <xsl:text>```&#10;</xsl:text>
        <!-- Don't apply Emphasis/etc templates in code blocks -->
        <xsl:for-each select="w:r">
            <xsl:value-of select="w:t" />
        </xsl:for-each>
        <xsl:text>&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'CodeB' or @w:val = 'CodeBWingding']]">
        <!-- Don't apply Emphasis/etc templates in code blocks -->
        <xsl:for-each select="w:r">
            <xsl:value-of select="w:t" />
        </xsl:for-each>
        <xsl:text>&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'CodeC' or @w:val = 'CodeCWingding']]">
        <!-- Don't apply Emphasis/etc templates in code blocks -->
        <xsl:for-each select="w:r">
            <xsl:value-of select="w:t" />
        </xsl:for-each>
        <xsl:text>&#10;```&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'CodeSingle']">
        <xsl:text>```&#10;</xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;```&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'ProductionDirective']">
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'Caption' or @w:val = 'TableTitle' or @w:val = 'Caption1' or @w:val = 'Listing']]">
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'BlockQuote']]">
        <xsl:text>> </xsl:text>
        <xsl:apply-templates select="*" />
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'BlockText']]">
        <xsl:text>&#10;</xsl:text>
        <xsl:text>> </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'Note']">
        <xsl:text>> </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p">
Unmatched: <xsl:value-of select="w:pPr/w:pStyle/@w:val" />
      <xsl:text>
      </xsl:text>


    </xsl:template>

    <!-- Character styles -->

    <xsl:template match="w:r[w:rPr/w:rStyle[@w:val = 'Literal' or @w:val = 'LiteralBold' or @w:val = 'LiteralCaption' or @w:val = 'LiteralBox']]">
        <xsl:choose>
            <xsl:when test="normalize-space(w:t) != ''">
                <xsl:if test="starts-with(w:t, ' ')">
                    <xsl:text> </xsl:text>
                </xsl:if>
                <xsl:text>`</xsl:text>
                <xsl:value-of select="normalize-space(w:t)" />
                <xsl:text>`</xsl:text>
                <xsl:if test="substring(w:t, string-length(w:t)) = ' '">
                    <xsl:text> </xsl:text>
                </xsl:if>
            </xsl:when>
            <xsl:when test="normalize-space(w:t) != w:t and w:t != ''">
                <xsl:text> </xsl:text>
            </xsl:when>
        </xsl:choose>
    </xsl:template>

    <xsl:template match="w:r[w:rPr/w:rStyle[@w:val = 'EmphasisBold']]">
        <xsl:choose>
            <xsl:when test="normalize-space(w:t) != ''">
                <xsl:if test="starts-with(w:t, ' ')">
                    <xsl:text> </xsl:text>
                </xsl:if>
                <xsl:text>**</xsl:text>
                <xsl:value-of select="normalize-space(w:t)" />
                <xsl:text>**</xsl:text>
                <xsl:if test="substring(w:t, string-length(w:t)) = ' '">
                    <xsl:text> </xsl:text>
                </xsl:if>
            </xsl:when>
            <xsl:when test="normalize-space(w:t) != w:t and w:t != ''">
                <xsl:text> </xsl:text>
            </xsl:when>
        </xsl:choose>
    </xsl:template>

    <xsl:template match="w:r[w:rPr/w:rStyle[@w:val = 'EmphasisItalic' or @w:val = 'EmphasisItalicBox' or @w:val = 'EmphasisNote' or @w:val = 'EmphasisRevCaption' or @w:val = 'EmphasisRevItal']]">
        <xsl:choose>
            <xsl:when test="normalize-space(w:t) != ''">
                <xsl:if test="starts-with(w:t, ' ')">
                    <xsl:text> </xsl:text>
                </xsl:if>
                <xsl:text>*</xsl:text>
                <xsl:value-of select="normalize-space(w:t)" />
                <xsl:text>*</xsl:text>
                <xsl:if test="substring(w:t, string-length(w:t)) = ' '">
                    <xsl:text> </xsl:text>
                </xsl:if>
            </xsl:when>
            <xsl:otherwise>
                <xsl:text> </xsl:text>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template match="w:r">
        <xsl:value-of select="w:t" />
    </xsl:template>
</xsl:stylesheet>
